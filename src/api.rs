use anyhow::Result;
use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio_stream::Stream;

use crate::config::Config;
use crate::history::{FunctionCall, Message, ToolCall};
use crate::tools;

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<tools::Tool>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,  // 最大输出 token 数，None 表示不限制
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

// 用于流式响应的 Message 结构体，允许字段为 null
#[derive(Debug, Deserialize)]
struct StreamMessage {
    #[allow(dead_code)]
    role: Option<String>,
    #[allow(dead_code)]
    content: Option<String>,
    #[allow(dead_code)]
    tool_calls: Option<serde_json::Value>,
    #[allow(dead_code)]
    function_calls: Option<serde_json::Value>,
    #[allow(dead_code)]
    reasoning_content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    delta: Option<Delta>,
    #[allow(dead_code)]
    message: Option<StreamMessage>,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Delta {
    #[allow(dead_code)]
    role: Option<String>,
    content: Option<String>,
    tool_calls: Option<Vec<ToolCallDelta>>,
    reasoning_content: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ToolCallDelta {
    #[allow(dead_code)]
    index: usize,
    id: Option<String>,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    tool_type: Option<String>,
    function: Option<FunctionDelta>,
}

#[derive(Debug, Deserialize)]
struct FunctionDelta {
    name: Option<String>,
    arguments: Option<String>,
}

pub struct ApiClient {
    client: Client,
    config: Config,
}

impl ApiClient {
    pub fn new(config: Config) -> Self {
        Self {
            client: Client::new(),
            config,
        }
    }

    pub async fn chat_stream(
        &self,
        messages: Vec<Message>,
    ) -> Result<Box<dyn Stream<Item = Result<StreamChunk>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", self.config.api_url);
        
        let request = ChatRequest {
            model: self.config.current_model.clone(),
            messages,
            tools: tools::get_available_tools(),
            stream: true,
            max_tokens: None,  // 不限制输出 token，使用模型默认值
        };

        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("API 错误 {}: {}", status, text);
        }

        let stream = response.bytes_stream();
        
        let mapped_stream = stream.filter_map(|chunk| async move {
            match chunk {
                Ok(bytes) => {
                    let text = String::from_utf8_lossy(&bytes);
                    parse_sse_chunk(&text)
                }
                Err(e) => Some(Err(anyhow::anyhow!("流错误: {}", e))),
            }
        });
        
        Ok(Box::new(Box::pin(mapped_stream)))
    }

    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/models", self.config.api_url);
        
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("无法获取模型列表");
        }

        #[derive(Deserialize)]
        struct ModelsResponse {
            data: Vec<ModelInfo>,
        }

        #[derive(Deserialize)]
        struct ModelInfo {
            id: String,
        }

        let models_response: ModelsResponse = response.json().await?;
        Ok(models_response.data.into_iter().map(|m| m.id).collect())
    }
}

#[derive(Debug, Clone)]
pub enum StreamChunk {
    Content(String),
    Reasoning(String),  // 思考过程
    ToolCall {
        id: String,
        name: String,
        arguments: String,
    },
    Done,
}

fn parse_sse_chunk(text: &str) -> Option<Result<StreamChunk>> {
    for line in text.lines() {
        if line.starts_with("data: ") {
            let data = &line[6..];
            
            if data.trim() == "[DONE]" {
                return Some(Ok(StreamChunk::Done));
            }

            match serde_json::from_str::<ChatResponse>(data) {
                Ok(response) => {
                    if let Some(choice) = response.choices.first() {
                        // 检查 finish_reason
                        if let Some(reason) = &choice.finish_reason {
                            if reason == "stop" || reason == "length" || reason == "tool_calls" {
                                return Some(Ok(StreamChunk::Done));
                            }
                        }
                        
                        if let Some(delta) = &choice.delta {
                            // 处理 tool_calls（最高优先级，因为最重要）
                            if let Some(tool_calls) = &delta.tool_calls {
                                for tc in tool_calls {
                                    let id = tc.id.as_deref().unwrap_or("");
                                    let name = tc.function.as_ref()
                                        .and_then(|f| f.name.as_deref())
                                        .unwrap_or("");
                                    let args = tc.function.as_ref()
                                        .and_then(|f| f.arguments.as_deref())
                                        .unwrap_or("");
                                    
                                    // 只要有任何内容就返回
                                    if !id.is_empty() || !name.is_empty() || !args.is_empty() {
                                        return Some(Ok(StreamChunk::ToolCall {
                                            id: id.to_string(),
                                            name: name.to_string(),
                                            arguments: args.to_string(),
                                        }));
                                    }
                                }
                            }
                            
                            // 处理 content（实际回复）
                            if let Some(content) = &delta.content {
                                if !content.is_empty() {
                                    return Some(Ok(StreamChunk::Content(content.clone())));
                                }
                            }
                            
                            // 处理 reasoning_content（思考过程）
                            if let Some(reasoning) = &delta.reasoning_content {
                                if !reasoning.is_empty() {
                                    return Some(Ok(StreamChunk::Reasoning(reasoning.clone())));
                                }
                            }
                        }
                    }
                }
                Err(_e) => {
                    // 静默忽略解析错误
                }
            }
        }
    }
    None
}

use crate::ui::{ToolCallDisplay, extract_key_argument};

pub struct ToolCallAccumulator {
    calls: std::collections::HashMap<String, (String, String)>,
    last_id: Option<String>,  // 记录上一个有效的 ID
    displays: std::collections::HashMap<String, ToolCallDisplay>,  // UI 显示组件
}

impl ToolCallAccumulator {
    pub fn new() -> Self {
        Self {
            calls: std::collections::HashMap::new(),
            last_id: None,
            displays: std::collections::HashMap::new(),
        }
    }

    pub fn add_chunk(&mut self, id: String, name: String, arguments: String) {
        // 如果 id 为空，使用上一个有效的 ID
        let key = if id.is_empty() {
            self.last_id.clone().unwrap_or_else(|| "temp".to_string())
        } else {
            self.last_id = Some(id.clone());  // 记录有效 ID
            id.clone()
        };
        
        let entry = self.calls.entry(key.clone()).or_insert((String::new(), String::new()));
        
        // 只在 name 非空时更新
        if !name.is_empty() {
            entry.0 = name.clone();
            // 创建 UI 显示组件
            if !self.displays.contains_key(&key) {
                self.displays.insert(key.clone(), ToolCallDisplay::new(name.clone()));
            }
        }
        
        // 追加 arguments
        if !arguments.is_empty() {
            entry.1.push_str(&arguments);
            
            // 尝试提取关键参数并更新 UI
            if let Some(display) = self.displays.get_mut(&key) {
                let tool_name = &entry.0;
                if let Some(arg) = extract_key_argument(tool_name, &entry.1) {
                    display.update_argument(arg);
                }
                display.render_streaming();
            }
        }
    }

    /// 获取所有 UI 显示组件
    pub fn get_displays(&self) -> &std::collections::HashMap<String, ToolCallDisplay> {
        &self.displays
    }

    pub fn into_tool_calls(self) -> Vec<ToolCall> {
        self.calls
            .into_iter()
            .filter_map(|(id, (name, arguments))| {
                // 过滤掉空的 tool call
                if name.is_empty() || arguments.is_empty() {
                    eprintln!("\x1b[33m[!] Warning:\x1b[0m Skipping empty tool call: id={}", id);
                    return None;
                }
                
                // 验证 JSON 是否完整且有效
                if !is_json_semantically_complete(&name, &arguments) {
                    let preview: String = arguments.chars().take(50).collect();
                    eprintln!("\x1b[33m[!] Warning:\x1b[0m Incomplete JSON for tool '{}': {}", name, preview);
                    return None;
                }
                
                // 再次验证是否可以解析
                let fixed_arguments = if serde_json::from_str::<serde_json::Value>(&arguments).is_err() {
                    // 尝试修复常见问题
                    let mut fixed = arguments.clone();
                    
                    // 1. 添加缺失的右花括号
                    let open_braces = fixed.matches('{').count();
                    let close_braces = fixed.matches('}').count();
                    if open_braces > close_braces {
                        for _ in 0..(open_braces - close_braces) {
                            fixed.push('}');
                        }
                    }
                    
                    // 2. 添加缺失的引号
                    if fixed.matches('"').count() % 2 != 0 {
                        fixed.push('"');
                    }
                    
                    // 3. 验证修复后的 JSON
                    if serde_json::from_str::<serde_json::Value>(&fixed).is_ok() {
                        eprintln!("\x1b[32m[✓] Info:\x1b[0m Auto-fixed JSON for tool '{}'", name);
                        fixed
                    } else {
                        eprintln!("\x1b[31m[✗] Error:\x1b[0m Failed to fix JSON for tool '{}'", name);
                        return None;
                    }
                } else {
                    arguments.clone()
                };
                
                Some(ToolCall {
                    id,
                    tool_type: "function".to_string(),
                    function: FunctionCall { name, arguments: fixed_arguments },
                })
            })
            .collect()
    }
}

pub async fn execute_tool_calls(
    tool_calls: &[ToolCall],
    working_dir: &Path,
    displays: &mut std::collections::HashMap<String, ToolCallDisplay>,
    require_approval: bool,
) -> Vec<Message> {
    let mut results = Vec::new();

    for tc in tool_calls {
        let tool_result = tools::execute_tool(
            &tc.function.name,
            &tc.function.arguments,
            working_dir,
            require_approval,
        )
        .unwrap_or_else(|e| tools::ToolResult::error(format!("工具执行错误: {}", e)));

        // 更新 UI 显示
        if let Some(display) = displays.get_mut(&tc.id) {
            display.finish(tool_result.success, Some(tool_result.brief.clone()));
            println!();  // 换行
            display.render_final();
        }

        results.push(Message {
            role: "tool".to_string(),
            content: tool_result.output,
            tool_calls: None,
            tool_call_id: Some(tc.id.clone()),
            name: Some(tc.function.name.clone()),
        });
    }

    results
}

/// 检查工具调用的 JSON 参数是否语义完整
fn is_json_semantically_complete(tool_name: &str, arguments: &str) -> bool {
    // 首先检查 JSON 结构完整性
    if !is_json_structurally_complete(arguments) {
        return false;
    }
    
    // 尝试解析为 JSON 对象
    let Ok(json) = serde_json::from_str::<serde_json::Value>(arguments) else {
        return false;
    };
    
    let Some(obj) = json.as_object() else {
        return false;
    };
    
    // 根据不同工具检查必需参数
    match tool_name {
        "file_read" | "file_write" => {
            // 必须有 path 参数，且不能为空
            obj.get("path")
                .and_then(|v| v.as_str())
                .map(|s| !s.is_empty())
                .unwrap_or(false)
        }
        "file_list" => {
            // file_list 的 path 是可选的，总是返回 true
            true
        }
        _ => {
            // 其他工具，只要 JSON 结构完整就行
            true
        }
    }
}

/// 检查 JSON 结构是否完整（括号和引号配对）
fn is_json_structurally_complete(s: &str) -> bool {
    let mut braces = 0;
    let mut brackets = 0;
    let mut in_string = false;
    let mut escape_next = false;
    
    for ch in s.chars() {
        if escape_next {
            escape_next = false;
            continue;
        }
        
        match ch {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '{' if !in_string => braces += 1,
            '}' if !in_string => braces -= 1,
            '[' if !in_string => brackets += 1,
            ']' if !in_string => brackets -= 1,
            _ => {}
        }
    }
    
    braces == 0 && brackets == 0 && !in_string
}

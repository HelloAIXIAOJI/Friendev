use anyhow::Result;
use futures::StreamExt;
use tokio_stream::Stream;
use std::io::Write;

use config::Config;
use history::{Message, ToolCall, FunctionCall};
use tools;
use ui::get_i18n;

// Use reqwest directly for now instead of full codex integration
use reqwest::Client;
use std::time::Duration;

use super::parser::parse_sse_line;
use super::stream::SseLineStream;
use super::types::{ChatRequest, ModelsResponse, StreamChunk};

/// Enhanced ApiClient that uses codex's robust patterns internally
/// while maintaining Friendev's simple interface
#[derive(Clone)]
pub struct ApiClient {
    config: Config,
    // Enhanced HTTP client with better configuration
    client: Client,
}

impl ApiClient {
    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(300)) // 5 minute timeout
            .connect_timeout(Duration::from_secs(60)) // 1 minute connect timeout
            .pool_idle_timeout(Duration::from_secs(90))
            .pool_max_idle_per_host(10)
            .user_agent(format!("friendev-enhanced/0.1.0"))
            .build()
            .unwrap_or_else(|_| Client::new());

        Self { config, client }
    }

    /// Clean message history: remove orphaned tool calls without responses
    /// (Keep Friendev's original logic)
    fn clean_messages(messages: &[Message]) -> Vec<Message> {
        let mut cleaned = Vec::new();
        let mut i = 0;

        while i < messages.len() {
            let msg = &messages[i];

            if msg.role == "assistant" && msg.tool_calls.is_some() {
                let tool_calls = msg.tool_calls.as_ref().unwrap();

                let tool_call_ids: std::collections::HashSet<_> =
                    tool_calls.iter().map(|tc| tc.id.clone()).collect();

                let mut has_responses = std::collections::HashSet::new();
                for msg in messages.iter().skip(i + 1) {
                    if msg.role == "tool" {
                        if let Some(tool_call_id) = &msg.tool_call_id {
                            if tool_call_ids.contains(tool_call_id) {
                                has_responses.insert(tool_call_id.clone());
                            }
                        }
                    } else if msg.role != "tool" {
                        break;
                    }
                }

                if has_responses.len() < tool_call_ids.len() {
                    let mut cleaned_msg = msg.clone();
                    if let Some(ref mut calls) = cleaned_msg.tool_calls {
                        calls.retain(|tc| has_responses.contains(&tc.id));

                        if calls.is_empty() {
                            cleaned_msg.tool_calls = None;
                        }
                    }

                    if cleaned_msg.tool_calls.is_some() {
                        cleaned.push(cleaned_msg);
                    }
                } else {
                    cleaned.push(msg.clone());
                }
            } else {
                cleaned.push(msg.clone());
            }

            i += 1;
        }

        cleaned
    }

    /// Enhanced retry logic inspired by codex
    async fn execute_with_retry<F, T>(&self, mut f: F) -> Result<T>
    where
        F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<T>> + Send>>,
    {
        let max_attempts = self.config.max_retries;
        let base_delay = Duration::from_millis(self.config.retry_delay_ms);

        for attempt in 0..=max_attempts {
            match f().await {
                Ok(result) => return Ok(result),
                Err(e) => {
                    if attempt == max_attempts {
                        return Err(e);
                    }

                    // Check if error is retryable
                    let should_retry = e.to_string().contains("timeout")
                        || e.to_string().contains("connection")
                        || e.to_string().contains("429")
                        || e.to_string().contains("529")
                        || e.to_string().contains("500")
                        || e.to_string().contains("502")
                        || e.to_string().contains("503")
                        || e.to_string().contains("504");

                    if !should_retry {
                        return Err(e);
                    }

                    // Exponential backoff with jitter
                    let delay_ms = base_delay.as_millis() as u64 * 2_u64.pow(attempt);
                    let jitter = fastrand::f64() * 0.1 + 0.9; // 0.9 to 1.0
                    let final_delay = Duration::from_millis((delay_ms as f64 * jitter) as u64);

                    let i18n = get_i18n();
                    println!(
                        "\n\x1b[33m[!] {} {}/{}... {}ms\x1b[0m",
                        i18n.get("api_retry_label"),
                        attempt + 1,
                        max_attempts + 1,
                        final_delay.as_millis()
                    );
                    tokio::time::sleep(final_delay).await;
                }
            }
        }
        
        // This line should never be reached due to the return statements above
        Err(anyhow::anyhow!("All retry attempts exhausted"))
    }

    /// Enhanced chat with better error handling and retry (non-streaming)
    pub async fn chat_with_retry(
        &self,
        messages: Vec<Message>,
        mcp_integration: Option<&mcp::McpIntegration>,
    ) -> Result<Message> {
        let cleaned_messages = Self::clean_messages(&messages);
        
        // Show streaming animation
        let spinner = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let start_time = std::time::Instant::now();
        
        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
        
        // Spawn spinner task
        let spinner_handle = tokio::spawn(async move {
            let mut i = 0;
            while rx.recv().await.is_none() {
                let elapsed = start_time.elapsed().as_secs();
                print!("\r\x1b[36m[Streaming {} [{}s]\x1b[0m", spinner[i % spinner.len()], elapsed);
                std::io::Write::flush(&mut std::io::stdout()).ok();
                i += 1;
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            // Clear the line when done
            print!("\r\x1b[K");
            std::io::Write::flush(&mut std::io::stdout()).ok();
        });
        
        let result = self.execute_with_retry(|| {
            let messages = cleaned_messages.clone();
            let tools = tools::get_available_tools_with_mcp(mcp_integration);
            let config = self.config.clone();
            let client = self.client.clone();

            Box::pin(async move {
                Self::chat_complete_static(&messages, tools, &config, client).await
            })
        }).await;
        
        // Stop spinner
        let _ = tx.send(()).await;
        let _ = spinner_handle.await;
        
        result
    }

    /// Static version of chat completion for use in retry
    async fn chat_complete_static(
        messages: &[Message],
        tools: Vec<tools::Tool>,
        config: &Config,
        client: Client,
    ) -> Result<Message> {
        let url = format!("{}/chat/completions", config.api_url);

        let request = ChatRequest {
            model: config.current_model.clone(),
            messages: messages.to_vec(),
            tools,
            stream: false,
            max_tokens: None,
            response_format: None,
        };

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .header("User-Agent", "friendev-enhanced/0.1.0")
            .timeout(Duration::from_secs(300)) // 5 minute timeout for completion
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            
            // Enhanced error reporting
            if status.as_u16() == 429 {
                anyhow::bail!("Rate limit exceeded. Please try again later.");
            } else if status.is_server_error() {
                anyhow::bail!("Server error {}: {}. This is a temporary issue.", status, text);
            } else {
                anyhow::bail!("API error {}: {}", status, text);
            }
        }

        let response_json: serde_json::Value = response.json().await?;
        
        // Parse message content
        let message = response_json["choices"][0]["message"].clone();
        
        let content = message["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        
        // Parse tool calls if present
        let tool_calls = if let Some(calls) = message.get("tool_calls").and_then(|v| v.as_array()) {
            let mut parsed_calls = Vec::new();
            for call in calls {
                if let Some(function) = call.get("function") {
                    parsed_calls.push(history::ToolCall {
                        id: call.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        tool_type: "function".to_string(),
                        function: history::FunctionCall {
                            name: function.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                            arguments: function.get("arguments").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                        },
                    });
                }
            }
            if parsed_calls.is_empty() { None } else { Some(parsed_calls) }
        } else {
            None
        };
        
        Ok(Message {
            role: "assistant".to_string(),
            content,
            tool_calls,
            tool_call_id: None,
            name: None,
        })
    }

    /// Static version of chat stream internal for use in retry
    async fn chat_stream_internal_static(
        messages: &[Message],
        tools: Vec<tools::Tool>,
        config: &Config,
        client: Client,
    ) -> Result<Box<dyn Stream<Item = Result<StreamChunk>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", config.api_url);

        let request = ChatRequest {
            model: config.current_model.clone(),
            messages: messages.to_vec(),
            tools,
            stream: true,
            max_tokens: None,
            response_format: None,
        };

        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", config.api_key))
            .header("Content-Type", "application/json")
            .header("User-Agent", "friendev-enhanced/0.1.0")
            .timeout(Duration::from_secs(60)) // Request timeout
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            
            // Enhanced error reporting
            if status.as_u16() == 429 {
                anyhow::bail!("Rate limit exceeded. Please try again later.");
            } else if status.is_server_error() {
                anyhow::bail!("Server error {}: {}. This is a temporary issue.", status, text);
            } else {
                anyhow::bail!("API error {}: {}", status, text);
            }
        }

        let stream = response.bytes_stream();
        let sse_stream = SseLineStream::new(stream);

        let mapped_stream = sse_stream.filter_map(|line_result| async move {
            match line_result {
                Ok(line) => parse_sse_line(&line),
                Err(e) => Some(Err(e)),
            }
        });

        Ok(Box::new(Box::pin(mapped_stream)))
    }

    /// Internal chat stream implementation
    async fn chat_stream_internal(
        &self,
        messages: Vec<Message>,
        tools: Vec<tools::Tool>,
        config: Config,
        client: Client,
    ) -> Result<Box<dyn Stream<Item = Result<StreamChunk>> + Unpin + Send>> {
        Self::chat_stream_internal_static(&messages, tools, &config, client).await
    }

    /// Keep other methods unchanged but with enhanced error handling
    pub async fn chat_complete(&self, messages: Vec<Message>, mcp_integration: Option<&mcp::McpIntegration>) -> Result<Message> {
        let url = format!("{}/chat/completions", self.config.api_url);
        
        let request = ChatRequest {
            model: self.config.current_model.clone(),
            messages,
            tools: tools::get_available_tools_with_mcp(mcp_integration),
            stream: false,
            max_tokens: Some(1000),
            response_format: None,
        };
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .header("User-Agent", "friendev-enhanced/0.1.0")
            .timeout(Duration::from_secs(120))
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("API error {}: {}", status, text);
        }
        
        let response_json: serde_json::Value = response.json().await?;
        
        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        
        Ok(Message {
            role: "assistant".to_string(),
            content,
            tool_calls: None,
            tool_call_id: None,
            name: None,
        })
    }

    pub async fn chat_complete_json(&self, messages: Vec<Message>) -> Result<Message> {
        let url = format!("{}/chat/completions", self.config.api_url);
        
        let request = ChatRequest {
            model: self.config.current_model.clone(),
            messages,
            tools: vec![],
            stream: false,
            max_tokens: Some(500),
            response_format: Some(super::types::ResponseFormat::JsonObject),
        };
        
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .header("User-Agent", "friendev-enhanced/0.1.0")
            .timeout(Duration::from_secs(60))
            .json(&request)
            .send()
            .await?;
        
        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await?;
            anyhow::bail!("API error {}: {}", status, text);
        }
        
        let response_json: serde_json::Value = response.json().await?;
        
        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();
        
        Ok(Message {
            role: "assistant".to_string(),
            content,
            tool_calls: None,
            tool_call_id: None,
            name: None,
        })
    }

    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/models", self.config.api_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("User-Agent", "friendev-enhanced/0.1.0")
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        if !response.status().is_success() {
            let i18n = get_i18n();
            anyhow::bail!(i18n.get("api_models_failed"));
        }

        let models_response: ModelsResponse = response.json().await?;
        Ok(models_response.data.into_iter().map(|m| m.id).collect())
    }
}
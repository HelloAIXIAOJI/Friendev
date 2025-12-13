use anyhow::Result;
use async_openai::{
    config::OpenAIConfig,
    types::chat::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestToolMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
        ChatCompletionTool, FunctionObjectArgs, FunctionCall, 
        ChatCompletionMessageToolCall, ResponseFormat,
    },
    Client,
};
use futures::{stream, Stream, StreamExt};

use config::Config;
use history::Message;
use tools;
use ui::get_i18n;

use super::types::StreamChunk;

#[derive(Clone)]
pub struct ApiClient {
    client: Client<OpenAIConfig>,
    config: Config,
}

impl ApiClient {
    pub fn new(config: Config) -> Self {
        let openai_config = OpenAIConfig::new()
            .with_api_key(config.api_key.clone())
            .with_api_base(config.api_url.clone());
            
        let client = Client::with_config(openai_config);

        Self { client, config }
    }

    /// Clean message history: remove orphaned tool calls without responses
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
    
    fn convert_messages(messages: Vec<Message>) -> Result<Vec<ChatCompletionRequestMessage>> {
        let mut req_messages = Vec::new();
        
        for msg in messages {
            let req_msg = match msg.role.as_str() {
                "system" => {
                    ChatCompletionRequestMessage::System(
                        ChatCompletionRequestSystemMessageArgs::default()
                            .content(msg.content)
                            .build()?
                    )
                },
                "user" => {
                    ChatCompletionRequestMessage::User(
                        ChatCompletionRequestUserMessageArgs::default()
                            .content(msg.content)
                            .build()?
                    )
                },
                "assistant" => {
                    let mut args = ChatCompletionRequestAssistantMessageArgs::default();
                    if !msg.content.is_empty() {
                        args.content(msg.content);
                    }
                    
                    if let Some(tool_calls) = msg.tool_calls {
                        let calls: Vec<async_openai::types::chat::ChatCompletionMessageToolCalls> = tool_calls.into_iter().map(|tc| {
                            async_openai::types::chat::ChatCompletionMessageToolCalls::Function(
                                ChatCompletionMessageToolCall {
                                    id: tc.id,
                                    function: FunctionCall {
                                        name: tc.function.name,
                                        arguments: tc.function.arguments,
                                    },
                                }
                            )
                        }).collect();
                        args.tool_calls(calls);
                    }
                    
                    ChatCompletionRequestMessage::Assistant(args.build()?)
                },
                "tool" => {
                    ChatCompletionRequestMessage::Tool(
                        ChatCompletionRequestToolMessageArgs::default()
                            .content(msg.content)
                            .tool_call_id(msg.tool_call_id.unwrap_or_default())
                            .build()?
                    )
                },
                _ => continue, 
            };
            req_messages.push(req_msg);
        }
        
        Ok(req_messages)
    }
    
    fn convert_tools(tools: Vec<tools::Tool>) -> Vec<ChatCompletionTool> {
        tools.into_iter().map(|t| {
            ChatCompletionTool {
                function: FunctionObjectArgs::default()
                    .name(t.function.name)
                    .description(t.function.description)
                    .parameters(t.function.parameters)
                    .build()
                    .unwrap(),
            }
        }).collect()
    }

    /// Stream chat with retry logic
    pub async fn chat_stream_with_retry(
        &self,
        messages: Vec<Message>,
        mcp_integration: Option<&mcp::McpIntegration>,
    ) -> Result<Box<dyn Stream<Item = Result<StreamChunk>> + Unpin + Send>> {
        let cleaned_messages = Self::clean_messages(&messages);

        let max_retries = self.config.max_retries;
        let base_delay = self.config.retry_delay_ms;

        for attempt in 0..=max_retries {
            if attempt > 0 {
                let delay = base_delay * (1 << (attempt - 1)); // exponential backoff
                let i18n = get_i18n();
                println!(
                    "\n\x1b[33m[!] {} {}/{}...{} {}ms\x1b[0m",
                    i18n.get("api_retry_label"),
                    attempt,
                    max_retries,
                    i18n.get("api_retry_waiting"),
                    delay
                );
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
            }

            match self.chat_stream(cleaned_messages.clone(), mcp_integration).await {
                Ok(stream) => return Ok(stream),
                Err(e) => {
                    if attempt == max_retries {
                        let i18n = get_i18n();
                        eprintln!("\n\x1b[31m[X] {}\x1b[0m", i18n.get("api_retries_failed"));
                        return Err(e);
                    }
                    let i18n = get_i18n();
                    eprintln!(
                        "\n\x1b[33m[!] {}: {}\x1b[0m",
                        i18n.get("api_request_failed"),
                        e
                    );
                }
            }
        }

        let i18n = get_i18n();
        Err(anyhow::anyhow!(i18n.get("api_retries_failed")))
    }

    /// Stream chat completions
    async fn chat_stream(
        &self,
        messages: Vec<Message>,
        mcp_integration: Option<&mcp::McpIntegration>,
    ) -> Result<Box<dyn Stream<Item = Result<StreamChunk>> + Unpin + Send>> {
        let converted_messages = Self::convert_messages(messages)?;
        let available_tools = tools::get_available_tools_with_mcp(mcp_integration);
        let converted_tools = if available_tools.is_empty() {
            None
        } else {
            let tools = Self::convert_tools(available_tools);
            let tools_enum: Vec<_> = tools.into_iter()
                .map(|t| async_openai::types::chat::ChatCompletionTools::Function(t))
                .collect();
            Some(tools_enum)
        };

        let mut request = CreateChatCompletionRequestArgs::default();
        request.model(self.config.current_model.clone())
            .messages(converted_messages);
            
        if let Some(tools) = converted_tools {
            request.tools(tools);
        }
            
        let request = request.build()?;

        let stream = self.client.chat().create_stream(request).await?;

        let mapped_stream = stream.flat_map(|result| {
            let mut items = Vec::new();
            match result {
                Ok(response) => {
                    if let Some(choice) = response.choices.first() {
                        let delta = &choice.delta;
                        if let Some(content) = &delta.content {
                            items.push(Ok(StreamChunk::Content(content.clone())));
                        }
                        
                        if let Some(tool_calls) = &delta.tool_calls {
                            if let Some(tool_call) = tool_calls.first() {
                                items.push(Ok(StreamChunk::ToolCall {
                                    id: tool_call.id.clone().unwrap_or_default(),
                                    name: tool_call.function.as_ref().map(|f| f.name.clone().unwrap_or_default()).unwrap_or_default(),
                                    arguments: tool_call.function.as_ref().map(|f| f.arguments.clone().unwrap_or_default()).unwrap_or_default(),
                                }));
                            }
                        }

                        if let Some(finish_reason) = &choice.finish_reason {
                             items.push(Ok(StreamChunk::FinishReason(format!("{:?}", finish_reason))));
                             items.push(Ok(StreamChunk::Done));
                        }
                    }
                },
                Err(e) => {
                    items.push(Err(anyhow::anyhow!("Stream error: {}", e)));
                }
            }
            stream::iter(items)
        });
        
        // Filter out empty content if needed, but for now we keep the mapping simple
        
        Ok(Box::new(Box::pin(mapped_stream)))
    }

    /// Non-streaming chat completion (for simple requests like prompt optimization)
    pub async fn chat_complete(&self, messages: Vec<Message>, mcp_integration: Option<&mcp::McpIntegration>) -> Result<Message> {
        let converted_messages = Self::convert_messages(messages)?;
        let available_tools = tools::get_available_tools_with_mcp(mcp_integration);
        let converted_tools = if available_tools.is_empty() {
            None
        } else {
            let tools = Self::convert_tools(available_tools);
            let tools_enum: Vec<_> = tools.into_iter()
                .map(|t| async_openai::types::chat::ChatCompletionTools::Function(t))
                .collect();
            Some(tools_enum)
        };
        
        let mut request = CreateChatCompletionRequestArgs::default();
        request.model(self.config.current_model.clone())
            .messages(converted_messages)
            .max_tokens(1000u16);
            
        if let Some(tools) = converted_tools {
            request.tools(tools);
        }

        let request = request.build()?;

        let response = self.client.chat().create(request).await?;
        
        let choice = response.choices.first().ok_or_else(|| anyhow::anyhow!("No choices in response"))?;
        let msg = &choice.message;
        
        Ok(Message {
            role: "assistant".to_string(),
            content: msg.content.clone().unwrap_or_default(),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        })
    }

    /// Non-streaming chat completion with JSON mode (for structured outputs like safety reviews)
    pub async fn chat_complete_json(&self, messages: Vec<Message>) -> Result<Message> {
        let converted_messages = Self::convert_messages(messages)?;
        
        let request = CreateChatCompletionRequestArgs::default()
            .model(self.config.current_model.clone())
            .messages(converted_messages)
            .max_tokens(500u16)
            .response_format(ResponseFormat::JsonObject)
            .build()?;

        let response = self.client.chat().create(request).await?;
        
        let choice = response.choices.first().ok_or_else(|| anyhow::anyhow!("No choices in response"))?;
        let msg = &choice.message;

        Ok(Message {
            role: "assistant".to_string(),
            content: msg.content.clone().unwrap_or_default(),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        })
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<String>> {
        // Note: The official async-openai library doesn't expose models() API
        // Return a default list of common models
        Ok(vec![
            "gpt-4o".to_string(),
            "gpt-4o-mini".to_string(),
            "gpt-4-turbo".to_string(),
            "gpt-4".to_string(),
            "gpt-3.5-turbo".to_string(),
        ])
    }
}

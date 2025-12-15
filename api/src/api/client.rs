use anyhow::Result;
use futures::StreamExt;
use reqwest::Client;
use tokio_stream::Stream;
use std::io::Write;

use config::Config;
use history::Message;
use tools;
use ui::get_i18n;

use super::parser::parse_sse_line;
use super::stream::SseLineStream;
use super::types::{ChatRequest, ModelsResponse, StreamChunk};

use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use std::time::Duration;

#[derive(Clone)]
pub struct ApiClient {
    client: Client,
    config: Config,
}

impl ApiClient {
    pub fn new(config: Config) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(300)) // 5 minute timeout
            .connect_timeout(std::time::Duration::from_secs(60)) // 1 minute connect timeout
            .build()
            .unwrap_or_else(|_| Client::new());

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

    /// Non-streaming chat with retry and animation
    pub async fn chat_with_retry(
        &self,
        messages: Vec<Message>,
        mcp_integration: Option<&mcp::McpIntegration>,
    ) -> Result<Message> {
        let cleaned_messages = Self::clean_messages(&messages);

        let max_retries = self.config.max_retries;
        let base_delay = self.config.retry_delay_ms;

        for attempt in 0..=max_retries {
            // Check for interrupt before each attempt
            if Self::check_interrupt()? {
                let i18n = get_i18n();
                return Err(anyhow::anyhow!("{}", i18n.get("hint_esc")));
            }

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
                
                // Check for interrupt during retry delay
                for _ in 0..(delay / 100) {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    if Self::check_interrupt()? {
                        let i18n = get_i18n();
                        return Err(anyhow::anyhow!("{}", i18n.get("hint_esc")));
                    }
                }
            }

            match self.chat_complete(cleaned_messages.clone(), mcp_integration).await {
                Ok(response) => return Ok(response),
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

    /// Non-streaming chat with retry and animation (for initial AI requests)
    pub async fn chat_with_retry_with_animation(
        &self,
        messages: Vec<Message>,
        mcp_integration: Option<&mcp::McpIntegration>,
    ) -> Result<Message> {
        let cleaned_messages = Self::clean_messages(&messages);
        
        // Show streaming animation for AI requests
        let spinner = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
        let start_time = std::time::Instant::now();
        
        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
        
        // Spawn spinner task - show at the bottom
        let spinner_handle = tokio::spawn(async move {
            let mut i = 0;
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                        let elapsed = start_time.elapsed().as_secs();
                        // Show at the bottom of output
                        println!("\r\x1b[36m[Streaming {} [{}s]\x1b[0m", spinner[i % spinner.len()], elapsed);
                        i += 1;
                    }
                    _ = rx.recv() => {
                        break;
                    }
                }
            }
        });

        let max_retries = self.config.max_retries;
        let base_delay = self.config.retry_delay_ms;

        for attempt in 0..=max_retries {
            // Check for interrupt before each attempt
            if Self::check_interrupt()? {
                let _ = tx.send(()).await; // Stop spinner
                spinner_handle.abort(); // Abort spinner task
                let i18n = get_i18n();
                return Err(anyhow::anyhow!("{}", i18n.get("hint_esc")));
            }

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
                
                // Check for interrupt during retry delay
                for _ in 0..(delay / 100) {
                    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                    if Self::check_interrupt()? {
                        let _ = tx.send(()).await; // Stop spinner
                        spinner_handle.abort(); // Abort spinner task
                        let i18n = get_i18n();
                        return Err(anyhow::anyhow!("{}", i18n.get("hint_esc")));
                    }
                }
            }

            match self.chat_complete(cleaned_messages.clone(), mcp_integration).await {
                Ok(response) => {
                    // Stop spinner and clear the line AFTER AI response is processed
                    let _ = tx.send(()).await;
                    spinner_handle.abort(); // Abort spinner task
                    
                    // Clear the streaming line and move to next line
                    println!("\r\x1b[K"); // Clear current line
                    
                    return Ok(response);
                }
                Err(e) => {
                    if attempt == max_retries {
                        let _ = tx.send(()).await; // Stop spinner
                        spinner_handle.abort(); // Abort spinner task
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

        let _ = tx.send(()).await; // Stop spinner
        spinner_handle.abort(); // Abort spinner task
        let i18n = get_i18n();
        Err(anyhow::anyhow!(i18n.get("api_retries_failed")))
    }

    /// Check if ESC key is pressed (non-blocking)
    fn check_interrupt() -> Result<bool> {
        // Poll with a slightly longer timeout for better key detection
        // 10ms is still fast enough to feel responsive but more reliable
        if poll(Duration::from_millis(10))? {
            if let Event::Key(key_event) = read()? {
                // Check for ESC key
                if key_event.code == KeyCode::Esc {
                    return Ok(true);
                }
                // Also check for Ctrl+C as alternative interrupt
                if key_event.code == KeyCode::Char('c') 
                    && key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    /// Stream chat completions
    async fn chat_stream(
        &self,
        messages: Vec<Message>,
        mcp_integration: Option<&mcp::McpIntegration>,
    ) -> Result<Box<dyn Stream<Item = Result<StreamChunk>> + Unpin + Send>> {
        let url = format!("{}/chat/completions", self.config.api_url);

        let request = ChatRequest {
            model: self.config.current_model.clone(),
            messages,
            tools: tools::get_available_tools_with_mcp(mcp_integration),
            stream: true,
            max_tokens: None,
            response_format: None,
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
            anyhow::bail!("API error {}: {}", status, text);
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

    /// Non-streaming chat completion (for simple requests like prompt optimization)
    pub async fn chat_complete(&self, messages: Vec<Message>, mcp_integration: Option<&mcp::McpIntegration>) -> Result<Message> {
        let url = format!("{}/chat/completions", self.config.api_url);

        let request = ChatRequest {
            model: self.config.current_model.clone(),
            messages,
            tools: tools::get_available_tools_with_mcp(mcp_integration),
            stream: false,
            max_tokens: None,  // Remove token limit for chat completion
            response_format: None,
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
            anyhow::bail!("API error {}: {}", status, text);
        }

        // Parse response
        let response_json: serde_json::Value = response.json().await?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        // Parse tool calls from response
        let tool_calls = response_json["choices"][0]["message"]["tool_calls"]
            .as_array()
            .and_then(|calls| {
                let mut parsed_calls = Vec::new();
                for call in calls {
                    let tool_call = history::ToolCall {
                        id: call["id"].as_str().unwrap_or("").to_string(),
                        tool_type: call["type"].as_str().unwrap_or("function").to_string(),
                        function: history::FunctionCall {
                            name: call["function"]["name"].as_str().unwrap_or("").to_string(),
                            arguments: call["function"]["arguments"].as_str().unwrap_or("").to_string(),
                        },
                    };
                    parsed_calls.push(tool_call);
                }
                Some(parsed_calls)
            });

        Ok(Message {
            role: "assistant".to_string(),
            content,
            tool_calls,
            tool_call_id: None,
            name: None,
        })
    }

    /// Non-streaming chat completion with JSON mode (for structured outputs like safety reviews)
    pub async fn chat_complete_json(&self, messages: Vec<Message>) -> Result<Message> {
        let url = format!("{}/chat/completions", self.config.api_url);
        
        let request = ChatRequest {
            model: self.config.current_model.clone(),
            messages,
            tools: vec![],  // No tools for JSON mode
            stream: false,
            max_tokens: Some(500),  // Limit tokens for safety reviews
            response_format: Some(super::types::ResponseFormat::JsonObject),
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
            anyhow::bail!("API error {}: {}", status, text);
        }
        
        // Parse response
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

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<String>> {
        let url = format!("{}/models", self.config.api_url);

        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
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

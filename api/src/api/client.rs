use anyhow::Result;
use futures::StreamExt;
use reqwest::Client;
use tokio_stream::Stream;

use crate::config::Config;
use crate::history::Message;
use crate::tools;
use crate::ui::get_i18n;

use super::parser::parse_sse_line;
use super::stream::SseLineStream;
use super::types::{ChatRequest, ModelsResponse, StreamChunk};

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
                for j in (i + 1)..messages.len() {
                    if messages[j].role == "tool" {
                        if let Some(tool_call_id) = &messages[j].tool_call_id {
                            if tool_call_ids.contains(tool_call_id) {
                                has_responses.insert(tool_call_id.clone());
                            }
                        }
                    } else if messages[j].role != "tool" {
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

            match self.chat_stream(cleaned_messages.clone()).await {
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
            max_tokens: None,
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

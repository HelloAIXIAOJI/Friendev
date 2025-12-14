use serde::{Deserialize, Serialize};

use tools;

/// Response format for chat requests
#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ResponseFormat {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "json_object")]
    JsonObject,
}

/// Chat request to be sent to the API
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<history::Message>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tools: Vec<tools::Tool>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

/// Chat API response
#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

/// Stream message for SSE responses (fields can be null)
#[derive(Debug, Deserialize)]
pub struct StreamMessage {
    #[allow(dead_code)]
    pub role: Option<String>,
    #[allow(dead_code)]
    pub content: Option<String>,
    #[allow(dead_code)]
    pub tool_calls: Option<serde_json::Value>,
    #[allow(dead_code)]
    pub function_calls: Option<serde_json::Value>,
    #[allow(dead_code)]
    pub reasoning_content: Option<String>,
}

/// Choice in API response
#[derive(Debug, Deserialize)]
pub struct Choice {
    pub delta: Option<Delta>,
    #[allow(dead_code)]
    pub message: Option<StreamMessage>,
    pub finish_reason: Option<String>,
}

/// Delta object in streaming response
#[derive(Debug, Deserialize)]
pub struct Delta {
    #[allow(dead_code)]
    pub role: Option<String>,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ToolCallDelta>>,
    pub reasoning_content: Option<String>,
}

/// Tool call delta in streaming response
#[derive(Debug, Deserialize)]
pub struct ToolCallDelta {
    #[allow(dead_code)]
    pub index: usize,
    pub id: Option<String>,
    #[serde(rename = "type")]
    #[allow(dead_code)]
    pub tool_type: Option<String>,
    pub function: Option<FunctionDelta>,
}

/// Function delta in tool call
#[derive(Debug, Deserialize)]
pub struct FunctionDelta {
    pub name: Option<String>,
    pub arguments: Option<String>,
}

/// Stream chunk - represents different types of streaming data
#[derive(Debug, Clone)]
pub enum StreamChunk {
    /// Text content from the model
    Content(String),
    /// Reasoning content (thinking process)
    Reasoning(String),
    /// Tool call data
    ToolCall {
        id: String,
        name: String,
        arguments: String,
    },
    /// Finish reason: stop, length, tool_calls, etc.
    FinishReason(String),
    /// Indicates stream is done
    Done,
}

/// Models list response
#[derive(Debug, Deserialize)]
pub struct ModelsResponse {
    pub data: Vec<ModelInfo>,
}

/// Model info from models list response
#[derive(Debug, Deserialize)]
pub struct ModelInfo {
    pub id: String,
}

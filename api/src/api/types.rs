/// Stream chunk - represents different types of streaming data
#[derive(Debug, Clone)]
pub enum StreamChunk {
    /// Text content from the model
    Content(String),
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

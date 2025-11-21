use super::output_formatter;
use super::stream_handler;
use crate::api::ApiClient;
use crate::history::{ChatSession, Message};
use crate::ui::ToolCallDisplay;
use anyhow::Result;
use std::collections::HashMap;

/// Send messages to AI and receive response
pub async fn send_and_receive(
    client: &ApiClient,
    messages: Vec<Message>,
    _session: &ChatSession,
) -> Result<(
    Message,
    Option<Vec<crate::history::ToolCall>>,
    HashMap<String, ToolCallDisplay>,
)> {
    // Use streaming request with retry
    let stream = client.chat_stream_with_retry(messages).await?;

    // Handle stream chunks
    let (content, tool_accumulator, has_tool_calls) =
        stream_handler::handle_stream_chunks(stream).await?;

    // Get tool calls and UI display components
    let displays = tool_accumulator.get_displays().clone();
    let tool_calls = if has_tool_calls {
        let calls = tool_accumulator.into_tool_calls();
        if calls.is_empty() {
            // Detected tool_call marker but all calls failed to parse
            output_formatter::print_tool_parse_error();
            None
        } else {
            Some(calls)
        }
    } else {
        None
    };

    let message = Message {
        role: "assistant".to_string(),
        content,
        tool_calls: tool_calls.clone(),
        tool_call_id: None,
        name: None,
    };

    Ok((message, tool_calls, displays))
}

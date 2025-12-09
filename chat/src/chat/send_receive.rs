use super::output_formatter;
use super::stream_handler;
use anyhow::Result;
use api::ApiClient;
use history::{ChatSession, Message};
use std::collections::HashMap;
use ui::ToolCallDisplay;

/// Send messages to AI and receive response
pub async fn send_and_receive(
    client: &ApiClient,
    messages: Vec<Message>,
    _session: &ChatSession,
    mcp_integration: Option<&mcp::McpIntegration>,
) -> Result<(
    Message,
    Option<Vec<history::ToolCall>>,
    HashMap<String, ToolCallDisplay>,
)> {
    // Use streaming request with retry
    let stream = client.chat_stream_with_retry(messages, mcp_integration).await?;

    // Handle stream chunks (with ESC interruption support)
    let (content, tool_accumulator, has_tool_calls, interrupted) =
        stream_handler::handle_stream_chunks(stream).await?;
    
    // If interrupted, return empty response
    if interrupted {
        let message = Message {
            role: "assistant".to_string(),
            content: content + "\n[生成已中断]",
            tool_calls: None,
            tool_call_id: None,
            name: None,
        };
        return Ok((message, None, HashMap::new()));
    }

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

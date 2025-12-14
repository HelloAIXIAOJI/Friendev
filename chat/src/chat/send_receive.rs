use super::output_formatter;
use super::stream_handler;
use anyhow::Result;
use api::ApiClient;
use history::{ChatSession, Message};
use std::collections::HashMap;
use std::io::Write;
use ui::ToolCallDisplay;

/// Send messages to AI and receive response
/// 
/// # Parameters
/// - `is_first_turn`: Whether this is the first turn (not a tool call loop iteration)
pub async fn send_and_receive(
    client: &api::ApiClient,
    messages: Vec<Message>,
    _session: &ChatSession,
    mcp_integration: Option<&mcp::McpIntegration>,
    is_first_turn: bool,
) -> Result<(
    Message,
    Option<Vec<history::ToolCall>>,
    HashMap<String, ToolCallDisplay>,
)> {
    // Use non-streaming request with retry and animation
    let response = client.chat_with_retry(messages, mcp_integration).await?;

    // Print AI prefix on first turn
    if is_first_turn {
        output_formatter::print_ai_prefix();
    }
    
    // Print the response content
    if !response.content.is_empty() {
        println!("{}", response.content);
        // Immediately flush to ensure prompt appears quickly
        std::io::Write::flush(&mut std::io::stdout()).ok();
    }
    
    // Get tool calls from response
    let tool_calls = response.tool_calls;
    let displays = HashMap::new(); // No streaming displays for non-streaming
    
    let tool_calls = if let Some(ref calls) = tool_calls {
        if calls.is_empty() {
            // Detected tool_call marker but all calls failed to parse
            output_formatter::print_tool_parse_error();
            None
        } else {
            Some(calls.clone())
        }
    } else {
        None
    };

    let message = Message {
        role: "assistant".to_string(),
        content: response.content,
        tool_calls: tool_calls.clone(),
        tool_call_id: None,
        name: None,
    };

    Ok((message, tool_calls, displays))
}

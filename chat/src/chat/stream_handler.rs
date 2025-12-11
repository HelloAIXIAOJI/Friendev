use super::output_formatter;
use anyhow::Result;
use api::{StreamChunk, ToolCallAccumulator};
use crossterm::event::{poll, read, Event, KeyCode, KeyModifiers};
use futures::StreamExt;
use std::time::Duration;

/// Process stream chunks and handle output with ESC key interruption support
/// 
/// # Parameters
/// - `stream`: The stream of chunks to process
/// - `print_prefix`: Whether to print the AI prefix (should be false for tool call loops)
pub async fn handle_stream_chunks(
    stream: impl futures::Stream<Item = Result<StreamChunk>> + Unpin,
    print_prefix: bool,
) -> Result<(String, ToolCallAccumulator, bool, bool)> {
    let mut stream = Box::pin(stream);

    let mut content = String::new();
    let mut tool_accumulator = ToolCallAccumulator::new();
    let mut has_tool_calls = false;
    let mut interrupted = false;

    let mut is_first_reasoning = true;
    let mut has_reasoning = false;

    if print_prefix {
        output_formatter::print_ai_prefix()?;
    }

    while let Some(chunk_result) = stream.next().await {
        // Check for ESC key press (non-blocking)
        if check_interrupt()? {
            interrupted = true;
            println!("\n\n\x1b[33m⚠ 已停止生成\x1b[0m\n");
            break;
        }
        match chunk_result? {
            StreamChunk::Content(text) => {
                output_formatter::print_content(&text, &mut has_reasoning)?;
                content.push_str(&text);
            }
            StreamChunk::Reasoning(text) => {
                output_formatter::print_reasoning(
                    &text,
                    &mut is_first_reasoning,
                    &mut has_reasoning,
                )?;
            }
            StreamChunk::ToolCall {
                id,
                name,
                arguments,
            } => {
                // If there was reasoning before, reset color and newline
                if has_reasoning {
                    print!("\x1b[0m\n\n");
                    has_reasoning = false;
                }
                if !has_tool_calls {
                    output_formatter::print_tool_call_separator()?;
                    has_tool_calls = true;
                }
                // Accumulate tool call data (will display in real-time)
                tool_accumulator.add_chunk(id, name, arguments);
            }
            StreamChunk::FinishReason(reason) => {
                // Record finish reason
                tool_accumulator.set_finish_reason(reason);
            }
            StreamChunk::Done => break,
        }
    }

    // Ensure color is reset at the end and newline
    if !interrupted {
        output_formatter::finalize_output(has_reasoning, content.is_empty())?;
    }

    Ok((content, tool_accumulator, has_tool_calls, interrupted))
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

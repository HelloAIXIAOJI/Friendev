use super::output_formatter;
use crate::api::{StreamChunk, ToolCallAccumulator};
use anyhow::Result;
use futures::StreamExt;

/// Process stream chunks and handle output
pub async fn handle_stream_chunks(
    stream: impl futures::Stream<Item = Result<StreamChunk>> + Unpin,
) -> Result<(String, ToolCallAccumulator, bool)> {
    let mut stream = Box::pin(stream);

    let mut content = String::new();
    let mut tool_accumulator = ToolCallAccumulator::new();
    let mut has_tool_calls = false;

    let mut is_first_reasoning = true;
    let mut has_reasoning = false;

    output_formatter::print_ai_prefix()?;

    while let Some(chunk_result) = stream.next().await {
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
    output_formatter::finalize_output(has_reasoning, content.is_empty())?;

    Ok((content, tool_accumulator, has_tool_calls))
}

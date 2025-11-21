use anyhow::Result;

use super::types::{ChatResponse, StreamChunk};

/// Parse a single SSE line (complete data)
pub fn parse_sse_line(line: &str) -> Option<Result<StreamChunk>> {
    let trimmed = line.trim();

    if trimmed.is_empty() || !trimmed.starts_with("data: ") {
        return None;
    }

    let data = &trimmed[6..];
    parse_sse_message(data)
}

/// Parse SSE message data
pub fn parse_sse_message(data: &str) -> Option<Result<StreamChunk>> {
    let data = data.trim();

    if data.is_empty() {
        return None;
    }

    if data == "[DONE]" {
        return Some(Ok(StreamChunk::Done));
    }

    match serde_json::from_str::<ChatResponse>(data) {
        Ok(response) => {
            if let Some(choice) = response.choices.first() {
                // Check finish_reason, emit as separate event
                if let Some(reason) = &choice.finish_reason {
                    if reason == "stop" || reason == "length" || reason == "tool_calls" {
                        return Some(Ok(StreamChunk::FinishReason(reason.clone())));
                    }
                }

                if let Some(delta) = &choice.delta {
                    // Handle tool_calls (highest priority)
                    if let Some(tool_calls) = &delta.tool_calls {
                        for tc in tool_calls {
                            let id = tc.id.as_deref().unwrap_or("");
                            let name = tc
                                .function
                                .as_ref()
                                .and_then(|f| f.name.as_deref())
                                .unwrap_or("");
                            let args = tc
                                .function
                                .as_ref()
                                .and_then(|f| f.arguments.as_deref())
                                .unwrap_or("");

                            // Return if any content
                            if !id.is_empty() || !name.is_empty() || !args.is_empty() {
                                return Some(Ok(StreamChunk::ToolCall {
                                    id: id.to_string(),
                                    name: name.to_string(),
                                    arguments: args.to_string(),
                                }));
                            }
                        }
                    }

                    // Handle content (actual response)
                    if let Some(content) = &delta.content {
                        if !content.is_empty() {
                            return Some(Ok(StreamChunk::Content(content.clone())));
                        }
                    }

                    // Handle reasoning_content (thinking process)
                    if let Some(reasoning) = &delta.reasoning_content {
                        if !reasoning.is_empty() {
                            return Some(Ok(StreamChunk::Reasoning(reasoning.clone())));
                        }
                    }
                }
            }
        }
        Err(_e) => {
            // SseLineStream correctly handles stream splitting
            // Errors here are mostly JSON structure errors, can be safely ignored
        }
    }
    None
}

/// Check if JSON structure is complete (brackets and quotes paired)
pub fn is_json_structurally_complete(s: &str) -> bool {
    let mut braces = 0;
    let mut brackets = 0;
    let mut in_string = false;
    let mut escape_next = false;

    for ch in s.chars() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '\\' if in_string => escape_next = true,
            '"' => in_string = !in_string,
            '{' if !in_string => braces += 1,
            '}' if !in_string => braces -= 1,
            '[' if !in_string => brackets += 1,
            ']' if !in_string => brackets -= 1,
            _ => {}
        }
    }

    braces == 0 && brackets == 0 && !in_string
}

/// Check if tool call JSON arguments are semantically complete
pub fn is_json_semantically_complete(tool_name: &str, arguments: &str) -> bool {
    // First check structural completeness
    if !is_json_structurally_complete(arguments) {
        return false;
    }

    // Try to parse as JSON object
    let Ok(json) = serde_json::from_str::<serde_json::Value>(arguments) else {
        return false;
    };

    let Some(obj) = json.as_object() else {
        return false;
    };

    // Check required parameters based on tool type
    match tool_name {
        "file_read" | "file_list" => {
            // path parameter required, must not be empty
            obj.get("path")
                .and_then(|v| v.as_str())
                .map(|s| !s.is_empty())
                .unwrap_or(true) // file_list path is optional
        }
        "file_write" => {
            // path parameter required
            let has_path = obj
                .get("path")
                .and_then(|v| v.as_str())
                .map(|s| !s.is_empty())
                .unwrap_or(false);

            if !has_path {
                return false;
            }

            // content parameter required
            obj.get("content")
                .and_then(|v| v.as_str())
                .map(|_| true)
                .unwrap_or(false)
        }
        "file_replace" => {
            // path and edits parameters required
            let has_path = obj
                .get("path")
                .and_then(|v| v.as_str())
                .map(|s| !s.is_empty())
                .unwrap_or(false);

            let has_edits = obj
                .get("edits")
                .and_then(|v| v.as_array())
                .map(|arr| !arr.is_empty())
                .unwrap_or(false);

            has_path && has_edits
        }
        _ => {
            // Other tools only need complete JSON structure
            true
        }
    }
}

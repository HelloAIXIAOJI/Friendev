
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

use std::path::Path;

use crate::history::{Message, ToolCall};
use crate::tools;
use crate::ui::get_i18n;
use crate::ui::ToolCallDisplay;

/// Execute tool calls and collect results
pub async fn execute_tool_calls(
    tool_calls: &[ToolCall],
    working_dir: &Path,
    displays: &mut std::collections::HashMap<String, ToolCallDisplay>,
    require_approval: bool,
) -> Vec<Message> {
    let mut results = Vec::new();

    for tc in tool_calls {
        // Skip invalid tool calls
        if tc.id.is_empty() || tc.function.name.is_empty() {
            let i18n = get_i18n();
            eprintln!(
                "\x1b[33m[!] {}:\x1b[0m {} id={}, name={}",
                i18n.get("warning"),
                i18n.get("api_skip_invalid_tool_call"),
                tc.id,
                tc.function.name
            );
            continue;
        }

        // Validate JSON arguments before execution
        if serde_json::from_str::<serde_json::Value>(&tc.function.arguments).is_err() {
            let i18n = get_i18n();
            eprintln!(
                "\x1b[33m[!] {}:\x1b[0m {} {}",
                i18n.get("warning"),
                i18n.get("api_skip_invalid_json_args"),
                tc.function.name
            );
            continue;
        }

        let tool_result = tools::execute_tool(
            &tc.function.name,
            &tc.function.arguments,
            working_dir,
            require_approval,
        )
        .await
        .unwrap_or_else(|e| {
            let i18n = get_i18n();
            let tmpl = i18n.get("api_tool_execution_error");
            let msg = tmpl.replace("{}", &e.to_string());
            tools::ToolResult::error(msg)
        });

        // Update UI display
        if let Some(display) = displays.get_mut(&tc.id) {
            display.finish(tool_result.success, Some(tool_result.brief.clone()));
            println!();
            display.render_final();
        }

        results.push(Message {
            role: "tool".to_string(),
            content: tool_result.message,
            tool_calls: None,
            tool_call_id: Some(tc.id.clone()),
            name: Some(tc.function.name.clone()),
        });
    }

    results
}

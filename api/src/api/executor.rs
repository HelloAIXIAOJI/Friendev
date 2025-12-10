use std::path::Path;

use history::{Message, ToolCall};
use tools::{self, ToolResult};
use ui::get_i18n;
use ui::ToolCallDisplay;
use futures::future::BoxFuture;
use anyhow::Result;

pub type CustomToolHandler = Box<dyn Fn(&str, &str, &Path) -> BoxFuture<'static, Result<Option<ToolResult>>> + Send + Sync>;

/// Execute tool calls and collect results
pub async fn execute_tool_calls(
    tool_calls: &[ToolCall],
    working_dir: &Path,
    displays: &mut std::collections::HashMap<String, ToolCallDisplay>,
    require_approval: bool,
    session_id: Option<&str>,
) -> Vec<Message> {
    execute_tool_calls_with_mcp(tool_calls, working_dir, displays, require_approval, session_id, None, None).await
}

/// Execute tool calls with MCP integration
pub async fn execute_tool_calls_with_mcp(
    tool_calls: &[ToolCall],
    working_dir: &Path,
    displays: &mut std::collections::HashMap<String, ToolCallDisplay>,
    require_approval: bool,
    session_id: Option<&str>,
    mcp_integration: Option<&mcp::McpIntegration>,
    custom_handler: Option<&CustomToolHandler>,
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

        // Try custom handler first
        let mut tool_result_opt = None;
        if let Some(handler) = custom_handler {
             match handler(&tc.function.name, &tc.function.arguments, working_dir).await {
                 Ok(Some(res)) => tool_result_opt = Some(res),
                 Ok(None) => {}, // Handler didn't handle it
                 Err(e) => {
                     // Handler failed
                     tool_result_opt = Some(tools::ToolResult::error(format!("Custom tool handler error: {}", e)));
                 }
             }
        }

        let tool_result = if let Some(res) = tool_result_opt {
            res
        } else {
            tools::execute_tool_with_mcp(
                &tc.function.name,
                &tc.function.arguments,
                working_dir,
                require_approval,
                session_id,
                mcp_integration,
            )
            .await
            .unwrap_or_else(|e| {
                let i18n = get_i18n();
                let tmpl = i18n.get("api_tool_execution_error");
                let msg = tmpl.replace("{}", &e.to_string());
                tools::ToolResult::error(msg)
            })
        };

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

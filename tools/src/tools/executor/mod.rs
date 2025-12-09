use anyhow::Result;
use std::path::Path;
use serde_json::Value;

use crate::tools::types::ToolResult;
use ui::get_i18n;

mod command_operations;
pub mod file_operations;
pub mod network_operations;
pub mod search_operations;
mod utils;
pub mod parser;
mod todo_operations;
pub mod lsp_client;

pub async fn execute_tool(
    name: &str,
    arguments: &str,
    working_dir: &Path,
    require_approval: bool,
    session_id: Option<&str>,
) -> Result<ToolResult> {
    execute_tool_with_mcp(name, arguments, working_dir, require_approval, session_id, None).await
}

pub async fn execute_tool_with_mcp(
    name: &str,
    arguments: &str,
    working_dir: &Path,
    require_approval: bool,
    session_id: Option<&str>,
    mcp_integration: Option<&mcp::McpIntegration>,
) -> Result<ToolResult> {
    match name {
        "file_list" => file_operations::execute_file_list(arguments, working_dir).await,
        "file_read" => file_operations::execute_file_read(arguments, working_dir).await,
        "file_search" => file_operations::execute_file_search(arguments, working_dir).await,
        "file_outline" => file_operations::execute_file_outline(arguments, working_dir).await,
        "file_search_by_outline" => {
            search_operations::execute_file_search_by_outline(arguments, working_dir).await
        }
        "index_file" => search_operations::execute_index_file(arguments, working_dir).await,
        "file_write" => {
            file_operations::execute_file_write(arguments, working_dir, require_approval).await
        }
        "file_replace" => {
            file_operations::execute_file_replace(arguments, working_dir, require_approval).await
        }
        "file_diff_edit" => {
            file_operations::execute_file_diff_edit(arguments, working_dir, require_approval).await
        }
        "network_search_auto" => search_operations::execute_search_auto(arguments).await,
        "network_search_duckduckgo" => {
            search_operations::execute_search_duckduckgo(arguments).await
        }
        "network_search_bing" => search_operations::execute_search_bing(arguments).await,
        "network_get_content" => network_operations::execute_fetch_content(arguments).await,
        "run_command" => command_operations::execute_run_command(arguments, require_approval).await,
        "todo_write" => todo_operations::execute_todo_write(arguments, working_dir, session_id).await,
        "todo_read" => todo_operations::execute_todo_read(arguments, working_dir, session_id).await,
        "mcp_resource_list" => {
            if let Some(integration) = mcp_integration {
                let args: serde_json::Value = serde_json::from_str(arguments).unwrap_or(serde_json::json!({}));
                let server_filter = args.get("mcp_server").and_then(|v| v.as_str());
                
                match integration.list_resources(server_filter).await {
                    Ok(resources) => {
                        let mut output = String::new();
                        output.push_str("Available MCP Resources:\n\n");
                        
                        for (server, res_list) in resources {
                            output.push_str(&format!("Server: {}\n", server));
                            for res in res_list {
                                output.push_str(&format!("- {} ({})\n", res.name, res.uri));
                                if let Some(desc) = res.description {
                                    output.push_str(&format!("  {}\n", desc));
                                }
                                if let Some(mime) = res.mime_type {
                                    output.push_str(&format!("  Type: {}\n", mime));
                                }
                            }
                            output.push_str("\n");
                        }
                        
                        Ok(ToolResult::ok("Listed MCP resources".to_string(), output))
                    }
                    Err(e) => Ok(ToolResult::error(format!("Failed to list resources: {}", e))),
                }
            } else {
                Ok(ToolResult::error("MCP integration not available".to_string()))
            }
        },
        "mcp_resource_read" => {
            if let Some(integration) = mcp_integration {
                let args: serde_json::Value = serde_json::from_str(arguments).unwrap_or(serde_json::json!({}));
                let uri = args.get("resource_uri").and_then(|v| v.as_str());
                let server_hint = args.get("mcp_server").and_then(|v| v.as_str());
                
                if let Some(uri) = uri {
                    match integration.read_resource(uri, server_hint).await {
                        Ok(content) => Ok(ToolResult::ok(format!("Read resource {}", uri), content)),
                        Err(e) => Ok(ToolResult::error(format!("Failed to read resource: {}", e))),
                    }
                } else {
                    Ok(ToolResult::error("Missing required argument: resource_uri".to_string()))
                }
            } else {
                Ok(ToolResult::error("MCP integration not available".to_string()))
            }
        },
        _ => {
            // Check if it's an MCP tool (format: server/tool)
            if let Some(integration) = mcp_integration {
                if let Some(pos) = name.find('/') {
                    let server = &name[..pos];
                    let tool = &name[pos + 1..];
                    
                    // Parse arguments from JSON string to Value
                    let args: serde_json::Value = match serde_json::from_str(arguments) {
                        Ok(v) => v,
                        Err(_) => serde_json::json!({}),
                    };
                    
                    match integration.call_tool(server, tool, args).await {
                        Ok(result) => Ok(ToolResult::ok("MCP tool executed".to_string(), result.to_string())),
                        Err(e) => Ok(ToolResult::error(format!("MCP tool error: {}", e))),
                    }
                } else {
                    // Try to find tool in any connected server
                    if let Ok(server_tools) = integration.get_available_tools().await {
                        for (server_name, tools) in server_tools {
                            if tools.contains(&name.to_string()) {
                                let args: serde_json::Value = match serde_json::from_str(arguments) {
                                    Ok(v) => v,
                                    Err(_) => serde_json::json!({}),
                                };
                                
                                match integration.call_tool(&server_name, name, args).await {
                                    Ok(result) => return Ok(ToolResult::ok("MCP tool executed".to_string(), result.to_string())),
                                    Err(e) => return Ok(ToolResult::error(format!("MCP tool error: {}", e))),
                                }
                            }
                        }
                    }
                    
                    let i18n = get_i18n();
                    let tmpl = i18n.get("tool_unknown");
                    Ok(ToolResult::error(tmpl.replace("{}", name)))
                }
            } else {
                let i18n = get_i18n();
                let tmpl = i18n.get("tool_unknown");
                Ok(ToolResult::error(tmpl.replace("{}", name)))
            }
        }
    }
}

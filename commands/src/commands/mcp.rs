use anyhow::Result;
use colored::Colorize;
use mcp::McpIntegration;
use i18n::I18n;

/// Handle MCP-related commands
pub async fn handle_mcp_command(input: &str, mcp_integration: Option<&McpIntegration>, i18n: &I18n) -> Result<()> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    
    if parts.len() < 2 {
        print_mcp_help(i18n);
        return Ok(());
    }

    match parts[1] {
        "status" | "list" => {
            handle_mcp_status(mcp_integration, i18n).await
        }
        "tools" => {
            if parts.len() >= 3 {
                handle_mcp_tools_for_server(mcp_integration, parts[2], i18n).await
            } else {
                handle_mcp_tools_all(mcp_integration, i18n).await
            }
        }
        "resources" => {
            if parts.len() >= 3 {
                handle_mcp_resources_for_server(mcp_integration, parts[2], i18n).await
            } else {
                handle_mcp_resources_all(mcp_integration, i18n).await
            }
        }
        "call" => {
            if parts.len() >= 4 {
                let server = parts[2];
                let tool = parts[3];
                let args = if parts.len() >= 5 {
                    serde_json::from_str(parts[4]).unwrap_or_else(|_| serde_json::json!({}))
                } else {
                    serde_json::json!({})
                };
                handle_mcp_call_tool(mcp_integration, server, tool, args, i18n).await
            } else {
                println!("{}", i18n.get("mcp_usage_call").yellow());
                Ok(())
            }
        }
        "read" => {
            if parts.len() >= 4 {
                let server = parts[2];
                let uri = parts[3];
                handle_mcp_read_resource(mcp_integration, server, uri, i18n).await
            } else {
                println!("{}", i18n.get("mcp_usage_read").yellow());
                Ok(())
            }
        }
        "help" => {
            print_mcp_help(i18n);
            Ok(())
        }
        _ => {
            println!("{} {}: {}", "❌".red(), i18n.get("mcp_unknown_command"), parts[1]);
            print_mcp_help(i18n);
            Ok(())
        }
    }
}

async fn handle_mcp_status(mcp_integration: Option<&McpIntegration>, i18n: &I18n) -> Result<()> {
    match mcp_integration {
        Some(integration) => {
            mcp::display_mcp_status(integration).await;
            Ok(())
        }
        None => {
            println!("{} {}", "❌".red(), i18n.get("mcp_not_available"));
            Ok(())
        }
    }
}

async fn handle_mcp_tools_all(mcp_integration: Option<&McpIntegration>, i18n: &I18n) -> Result<()> {
    match mcp_integration {
        Some(integration) => {
            println!("🔧 {}:", i18n.get("mcp_available_tools").cyan().bold());
            
            match integration.get_available_tools().await {
                Ok(server_tools) => {
                    if server_tools.is_empty() {
                        println!("  {}", "No tools available".dimmed());
                    } else {
                        for (server, tools) in server_tools {
                            println!("\n  {} {}", "📡".cyan(), server.cyan().bold());
                            for tool in tools {
                                println!("    • {}", tool);
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("{} Failed to get tools: {}", "❌".red(), e);
                }
            }
            Ok(())
        }
        None => {
            println!("{} {}", "❌".red(), i18n.get("mcp_not_available"));
            Ok(())
        }
    }
}

async fn handle_mcp_tools_for_server(mcp_integration: Option<&McpIntegration>, server: &str, i18n: &I18n) -> Result<()> {
    match mcp_integration {
        Some(integration) => {
            println!("{} {} '{}':", "🔧".cyan(), i18n.get("mcp_tools_for_server"), server.cyan());
            
            match integration.get_available_tools().await {
                Ok(server_tools) => {
                    if let Some(tools) = server_tools.get(server) {
                        if tools.is_empty() {
                            println!("  {}", i18n.get("mcp_no_tools").dimmed());
                        } else {
                            for tool in tools {
                                println!("  • {}", tool);
                            }
                        }
                    } else {
                        println!("{} {}: '{}'", "❌".red(), i18n.get("mcp_server_not_found"), server);
                    }
                }
                Err(e) => {
                    println!("{} Failed to get tools: {}", "❌".red(), e);
                }
            }
            Ok(())
        }
        None => {
            println!("{} {}", "❌".red(), i18n.get("mcp_not_available"));
            Ok(())
        }
    }
}

async fn handle_mcp_resources_all(mcp_integration: Option<&McpIntegration>, i18n: &I18n) -> Result<()> {
    match mcp_integration {
        Some(_integration) => {
            println!("📁 {}:", i18n.get("mcp_available_resources").cyan().bold());
            println!("  {}", i18n.get("mcp_resource_not_implemented").dimmed());
            // TODO: Implement resource listing when available
            Ok(())
        }
        None => {
            println!("{} {}", "❌".red(), i18n.get("mcp_not_available"));
            Ok(())
        }
    }
}

async fn handle_mcp_resources_for_server(_mcp_integration: Option<&McpIntegration>, server: &str, i18n: &I18n) -> Result<()> {
    println!("📁 {} '{}':", i18n.get("mcp_resources_for_server"), server.cyan());
    println!("  {}", i18n.get("mcp_resource_not_implemented").dimmed());
    // TODO: Implement resource listing when available
    Ok(())
}

async fn handle_mcp_call_tool(mcp_integration: Option<&McpIntegration>, server: &str, tool: &str, args: serde_json::Value, i18n: &I18n) -> Result<()> {
    match mcp_integration {
        Some(integration) => {
            println!("{} {}", "🚀".cyan(), i18n.get("mcp_calling_tool_msg").replace("{}", tool).replace("{}", server));
            
            match integration.call_tool(server, tool, args).await {
                Ok(result) => {
                    println!("{} {}:", "✅".green(), i18n.get("mcp_tool_result"));
                    println!("{}", serde_json::to_string_pretty(&result)?);
                }
                Err(e) => {
                    println!("{} {}: {}", "❌".red(), i18n.get("mcp_tool_failed"), e);
                }
            }
            Ok(())
        }
        None => {
            println!("{} {}", "❌".red(), i18n.get("mcp_not_available"));
            Ok(())
        }
    }
}

async fn handle_mcp_read_resource(mcp_integration: Option<&McpIntegration>, server: &str, uri: &str, i18n: &I18n) -> Result<()> {
    match mcp_integration {
        Some(integration) => {
            println!("{} {}", "📖".cyan(), i18n.get("mcp_reading_resource_msg").replace("{}", uri).replace("{}", server));
            
            match integration.read_resource(uri, Some(server)).await {
                Ok(content) => {
                    println!("{} {}:", "✅".green(), i18n.get("mcp_resource_content"));
                    println!("{}", content);
                }
                Err(e) => {
                    println!("{} {}: {}", "❌".red(), i18n.get("mcp_resource_failed"), e);
                }
            }
            Ok(())
        }
        None => {
            println!("{} {}", "❌".red(), i18n.get("mcp_not_available"));
            Ok(())
        }
    }
}

fn print_mcp_help(i18n: &I18n) {
    println!("🔗 {}:", i18n.get("mcp_commands_help").cyan().bold());
    println!("  {} - {}", "mcp status".green(), i18n.get("mcp_status"));
    println!("  {} - {}", "mcp tools".green(), i18n.get("mcp_tools"));
    println!("  {} - {}", "mcp tools <server>".green(), i18n.get("mcp_tools_server"));
    println!("  {} - {}", "mcp resources".green(), i18n.get("mcp_resources"));
    println!("  {} - {}", "mcp resources <server>".green(), i18n.get("mcp_resources_server"));
    println!("  {} - {}", "mcp call <server> <tool> [args]".green(), i18n.get("mcp_call_tool"));
    println!("  {} - {}", "mcp read <server> <uri>".green(), i18n.get("mcp_read_resource"));
    println!("  {} - {}", "mcp help".green(), i18n.get("mcp_help"));
    println!();
    println!("{}:", i18n.get("mcp_examples").yellow());
    println!("  {}", "mcp status".dimmed());
    println!("  {}", "mcp tools filesystem".dimmed());
    println!("  {}", "mcp call filesystem list_files '{\"path\": \"/tmp\"}'".dimmed());
    println!("  {}", "mcp read github file://README.md".dimmed());
}

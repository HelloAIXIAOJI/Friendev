use anyhow::Result;
use api::{ApiClient, CustomToolHandler};
use config::Config;
use history::{ChatSession, Message};
use mcp::McpIntegration;
use ui::get_i18n;
use super::message_builder;
use super::send_receive;
use tools::ToolResult;

/// Run the agent loop: send message, handle tool calls, and repeat until done
pub fn run_agent_loop<'a>(
    api_client: &'a ApiClient,
    config: &'a Config,
    session: &'a mut ChatSession,
    mcp_integration: Option<&'a McpIntegration>,
    auto_approve: bool,
    subagent_type: Option<String>,
) -> futures::future::BoxFuture<'a, Result<bool>> {
    Box::pin(async move {
        let mut messages = message_builder::build_messages_with_agents_md(session, config, mcp_integration, subagent_type.as_deref())?;
        
        // Define custom handler for "task" tool
        let client_clone = api_client.clone();
        let config_clone = config.clone();
        let mcp_clone = mcp_integration.cloned();
        
        let custom_handler: CustomToolHandler = Box::new(move |name, args, working_dir| {
            let client = client_clone.clone();
            let config = config_clone.clone();
            let mcp = mcp_clone.clone();
            let working_dir = working_dir.to_path_buf();
            let name = name.to_string();
            let args = args.to_string();
            
            Box::pin(async move {
                if name != "task" {
                    return Ok(None);
                }
                
                // Parse arguments
                let args_json: serde_json::Value = serde_json::from_str(&args)?;
                let _description = args_json.get("description").and_then(|v| v.as_str()).unwrap_or("Subtask");
                let prompt = args_json.get("prompt").and_then(|v| v.as_str()).ok_or_else(|| anyhow::anyhow!("Missing prompt"))?;
                let subagent_type_str = args_json.get("subagent_type").and_then(|v| v.as_str()).unwrap_or("general");
                
                // Create sub-session
                let mut sub_session = ChatSession::new(working_dir);
                
                // Add user message
                sub_session.add_message(Message {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                    tool_calls: None,
                    tool_call_id: None,
                    name: None,
                });
                
                println!("\n\x1b[36m🤖 Starting subagent: {}\x1b[0m", subagent_type_str);
                
                // Run loop (recursive)
                // Inherit auto_approve status from parent session
                let success = run_agent_loop(&client, &config, &mut sub_session, mcp.as_ref(), auto_approve, Some(subagent_type_str.to_string())).await?;
                
                // Extract result
                let result_content = if success {
                    sub_session.messages.iter().rev()
                        .find(|m| m.role == "assistant")
                        .map(|m| m.content.clone())
                        .unwrap_or_else(|| "Subagent completed but returned no content.".to_string())
                } else {
                    "Subagent failed to complete the task.".to_string()
                };
                
                println!("\n\x1b[36m🤖 Subagent finished\x1b[0m");
                
                Ok(Some(ToolResult::ok(format!("Subagent '{}' completed", subagent_type_str), result_content)))
            })
        });

        let mut is_first_turn = true;
        loop {
            match send_receive::send_and_receive(api_client, messages.clone(), session, mcp_integration, is_first_turn).await {
                Ok((response_msg, tool_calls, mut displays)) => {
                    // After first turn, subsequent turns are tool call loops
                    is_first_turn = false;
                    session.add_message(response_msg);
                    
                    // Save session immediately after receiving AI response
                    if let Err(e) = session.save() {
                        let i18n = get_i18n();
                        // Use a fallback message if key doesn't exist yet (though we added it)
                        let msg = i18n.get("history_save_error");
                        let msg = if msg == "history_save_error" {
                            format!("Warning: Failed to save session history: {}", e)
                        } else {
                            msg.replace("{}", &e.to_string())
                        };
                        eprintln!("\n\x1b[33m[!] {}\x1b[0m", msg);
                    }

                    if let Some(calls) = tool_calls {
                        // Execute tool calls
                        let tool_results = api::execute_tool_calls_with_mcp(
                            &calls,
                            &session.working_directory,
                            &mut displays,
                            !auto_approve,
                            Some(&session.id.to_string()),
                            mcp_integration,
                            Some(&custom_handler),
                        ).await;

                        for result in tool_results {
                            session.add_message(result);
                        }
                        
                        // Save session immediately after tool execution results are added
                        if let Err(e) = session.save() {
                            let i18n = get_i18n();
                            let msg = i18n.get("history_save_error");
                            let msg = if msg == "history_save_error" {
                                format!("Warning: Failed to save session history: {}", e)
                            } else {
                                msg.replace("{}", &e.to_string())
                            };
                            eprintln!("\n\x1b[33m[!] {}\x1b[0m", msg);
                        }

                        // Rebuild messages with new history
                        messages = message_builder::build_messages_with_agents_md(session, config, mcp_integration, subagent_type.as_deref())?;
                        continue;
                    }

                    return Ok(true);
                }
                Err(e) => {
                    let i18n = get_i18n();
                    eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("api_error"), e);
                    // Remove last message since no valid response
                    if !session.messages.is_empty() {
                        session.messages.pop();
                    }
                    return Ok(false);
                }
            }
        }
    })
}

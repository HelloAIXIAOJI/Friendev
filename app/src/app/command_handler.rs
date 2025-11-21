use super::message_builder;
use super::startup::AppState;
use crate::api;
use crate::chat;
use crate::commands;
use crate::history::Message;
use crate::security;
use crate::ui::get_i18n;
use anyhow::Result;

/// Handle user input and command processing
pub async fn handle_user_input(line: &str, state: &mut AppState) -> Result<()> {
    // Handle commands
    if line.starts_with('/') {
        // Special handling for /agents.md command
        if line == "/agents.md" {
            handle_agents_md_command(state).await?;
        } else {
            // Other commands
            if let Err(e) = commands::handle_command(
                line,
                &mut state.config,
                &mut state.session,
                &mut state.api_client,
            )
            .await
            {
                let i18n = get_i18n();
                eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), e);
            }
        }
        return Ok(());
    }

    // Security check: intercept suspicious input
    if security::is_input_suspicious(line) {
        let i18n = get_i18n();
        eprintln!(
            "\n\x1b[31m[X] {}:\x1b[0m {}\n",
            i18n.get("security_warning_label"),
            i18n.get("security_forbidden_tokens")
        );
        return Ok(());
    }

    // User message
    let user_message = Message {
        role: "user".to_string(),
        content: line.to_string(),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    };
    state.session.add_message(user_message);

    // Process chat and tool calls
    process_chat_loop(state).await?;

    state.session.save()?;
    Ok(())
}

/// Handle /agents.md command
async fn handle_agents_md_command(state: &mut AppState) -> Result<()> {
    match commands::handle_agents_md_command(&state.session, &state.i18n).await {
        Ok(analysis_prompt) => {
            // Add prompt as USER message to session
            let analysis_message = Message {
                role: "user".to_string(),
                content: analysis_prompt,
                tool_calls: None,
                tool_call_id: None,
                name: None,
            };
            state.session.add_message(analysis_message);

            // Auto-send to AI (same flow as normal user message)
            process_chat_loop(state).await?;
            state.session.save()?;
        }
        Err(e) => eprintln!("\n\x1b[31m[X] Error:\x1b[0m {}\n", e),
    }
    Ok(())
}

/// Process chat loop: send message and handle tool calls
async fn process_chat_loop(state: &mut AppState) -> Result<()> {
    let mut messages =
        message_builder::build_messages_with_agents_md(&state.session, &state.config)?;

    loop {
        match chat::send_and_receive(&state.api_client, messages.clone(), &state.session).await {
            Ok((response_msg, tool_calls, mut displays)) => {
                state.session.add_message(response_msg);

                if let Some(calls) = tool_calls {
                    // Execute tool calls (approval based on --ally flag)
                    let tool_results = api::execute_tool_calls(
                        &calls,
                        &state.session.working_directory,
                        &mut displays,
                        !state.auto_approve, // If --ally is set, no approval needed
                    )
                    .await;

                    for result in tool_results {
                        state.session.add_message(result);
                    }

                    // Continue loop to send tool results to AI
                    messages = message_builder::build_messages_with_agents_md(
                        &state.session,
                        &state.config,
                    )?;
                    continue;
                }

                break;
            }
            Err(e) => {
                let i18n = get_i18n();
                eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("api_error"), e);
                // Remove last message since no valid response
                if !state.session.messages.is_empty() {
                    state.session.messages.pop();
                }
                break;
            }
        }
    }
    Ok(())
}

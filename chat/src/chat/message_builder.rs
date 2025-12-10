use anyhow::Result;
use config::Config;
use history::{ChatSession, Message};
use prompts;

/// Build message sequence with SYSTEM prompt and history
/// AGENTS.md is integrated in the system prompt (loaded in real-time)
pub fn build_messages_with_agents_md(
    session: &ChatSession,
    config: &Config,
    mcp_integration: Option<&mcp::McpIntegration>,
    subagent_type: Option<&str>,
) -> Result<Vec<Message>> {
    let system_prompt = if let Some(type_) = subagent_type {
        prompts::get_subagent_system_prompt(
            &config.ai_language,
            &config.current_model,
            &session.working_directory,
            mcp_integration,
            type_,
        )
    } else {
        prompts::get_system_prompt(
            &config.ai_language,
            &config.current_model,
            &session.working_directory,
            mcp_integration,
        )
    };

    let mut messages = vec![Message {
        role: "system".to_string(),
        content: system_prompt,
        tool_calls: None,
        tool_call_id: None,
        name: None,
    }];

    // Add history messages
    messages.extend(session.messages.clone());

    Ok(messages)
}

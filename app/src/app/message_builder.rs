use crate::config::Config;
use crate::history::{ChatSession, Message};
use crate::prompts;
use anyhow::Result;

/// Build message sequence with SYSTEM prompt and history
/// AGENTS.md is integrated in the system prompt (loaded in real-time)
pub fn build_messages_with_agents_md(
    session: &ChatSession,
    config: &Config,
) -> Result<Vec<Message>> {
    let mut messages = vec![Message {
        role: "system".to_string(),
        content: prompts::get_system_prompt(
            &config.ai_language,
            &config.current_model,
            &session.working_directory,
        ),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    }];

    // Add history messages
    messages.extend(session.messages.clone());

    Ok(messages)
}

mod commands;

use anyhow::Result;
use api::ApiClient;
use config::Config;
use history::ChatSession;
use mcp::McpIntegration;

pub use commands::{handle_agents_md_command, print_help, handle_command_with_parts};

/// Handle commands that start with /
pub async fn handle_command(
    input: &str,
    config: &mut Config,
    session: &mut ChatSession,
    api_client: &mut ApiClient,
) -> Result<()> {
    handle_command_with_mcp(input, config, session, api_client, None).await
}

/// Handle commands with MCP integration
pub async fn handle_command_with_mcp(
    input: &str,
    config: &mut Config,
    session: &mut ChatSession,
    api_client: &mut ApiClient,
    mcp_integration: Option<&McpIntegration>,
) -> Result<()> {
    // Handle /todo commands specially
    if input.starts_with("/todo") {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let i18n = i18n::I18n::new(&config.ui_language);
        commands::todo::handle_todo_command(&parts, &i18n, session)?;
        return Ok(());
    }

    // Handle /mcp commands specially
    if input.starts_with("/mcp") {
        let i18n = i18n::I18n::new(&config.ui_language);
        commands::mcp::handle_mcp_command(input, mcp_integration, &i18n).await?;
        return Ok(());
    }

    // Handle /prompt commands specially
    if input.starts_with("/prompt") {
        let i18n = i18n::I18n::new(&config.ui_language);
        commands::prompt::handle_prompt_command(mcp_integration, &i18n, config, session, api_client).await?;
        return Ok(());
    }

    // Handle other commands using the old signature
    let parts: Vec<&str> = input.split_whitespace().collect();
    commands::handle_command_with_parts(&parts, config, session, api_client).await
}

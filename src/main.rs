mod agents;
mod api;
mod app;
mod chat;
mod commands;
mod config;
mod history;
mod i18n;
mod prompts;
mod search_tool;
mod security;
mod tools;
mod ui;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize application
    let state = app::initialize_app().await?;
    
    // Run REPL loop
    app::run_repl(state).await?;
    
    Ok(())
}

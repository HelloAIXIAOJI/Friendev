mod help;
mod commands;

use std::io::{self, Write};
use anyhow::Result;
use colored::Colorize;
use tokio::io::{AsyncBufReadExt, BufReader};

use crate::client::ClientManager;
use crate::config::McpConfig;

pub async fn run() -> Result<()> {
    let mut manager = ClientManager::new();
    run_with_manager_internal(&mut manager, true).await
}

pub async fn run_with_manager(manager: ClientManager) -> Result<()> {
    let mut manager = manager;
    run_with_manager_internal(&mut manager, false).await
}

async fn run_with_manager_internal(manager: &mut ClientManager, load_config: bool) -> Result<()> {

    help::print_banner();

    if load_config {
        println!("{}", "Loading configuration...".dimmed());
        match McpConfig::load() {
            Ok(config) => {
                manager.load_from_config(&config).await?;
            }
            Err(e) => {
                eprintln!("{} {}", "Failed to load config:".red(), e);
            }
        }
    }

    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut line = String::new();

    loop {
        print!("{} ", ">".cyan().bold());
        io::stdout().flush()?;

        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;
        if bytes_read == 0 {
            break;
        }

        let input = line.trim();
        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        match parts[0] {
            "/reload" => commands::handle_reload(manager).await?,
            "/list" => commands::handle_list(&manager, &parts).await?,
            "/call" => commands::handle_call(&manager, input, &parts).await?,
            "/read" => commands::handle_read(&manager, &parts).await?,
            "/down" => commands::handle_down(&manager, &parts).await?,
            "/info" => commands::handle_info(&manager, &parts).await?,
            "/use" => commands::handle_use(&manager, &parts).await?,
            "/newconfig" => commands::handle_newconfig(&parts),
            "/exit" | "/quit" => {
                println!("{}", "Goodbye!".cyan());
                break;
            }
            _ => commands::handle_unknown(parts[0]),
        }
    }

    Ok(())
}

mod config;
mod client;
mod repl;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mcpcs-client")]
#[command(about = "MCP Client with multiple transport options")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Interactive REPL mode
    Repl,
    /// Connect to SSE server
    Sse {
        /// SSE server URL
        #[arg(short, long)]
        url: String,
        /// Server name for display
        #[arg(short, long, default_value = "sse-server")]
        name: String,
    },
    /// Connect to HTTP server
    Http {
        /// HTTP server URL
        #[arg(short, long)]
        url: String,
        /// Server name for display
        #[arg(short, long, default_value = "http-server")]
        name: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Repl) {
        Commands::Repl => {
            repl::run().await
        }
        Commands::Sse { url, name } => {
            use crate::client::ClientManager;
            use crate::config::McpServerConfig;
            use colored::Colorize;

            let mut manager = ClientManager::new();
            
            // Create SSE config
            let sse_config = McpServerConfig::Sse { 
                transport: crate::config::SseTransport::Sse,
                url: url.clone(),
                auth_token: None,  // Can be extended to accept from CLI
                headers: None,     // Can be extended to accept from CLI
            };
            
            println!("{} {}", "Connecting to SSE server:".green(), url.cyan());
            
            match manager.connect(&sse_config).await {
                Ok(client) => {
                    manager.clients.insert(name.clone(), std::sync::Arc::new(client));
                    println!("{} {}", "Connected to SSE server:".green(), name.cyan());
                    
                    // Start interactive mode with this SSE connection
                    repl::run_with_manager(manager).await
                }
                Err(e) => {
                    eprintln!("{} {}", "Failed to connect to SSE server:".red(), e);
                    std::process::exit(1);
                }
            }
        }
        Commands::Http { url, name } => {
            use crate::client::ClientManager;
            use crate::config::McpServerConfig;
            use colored::Colorize;

            let mut manager = ClientManager::new();
            
            // Create HTTP config
            let http_config = McpServerConfig::Http { 
                transport: crate::config::HttpTransport::Http,
                url: url.clone(),
                auth_token: None,  // Can be extended to accept from CLI
                headers: None,     // Can be extended to accept from CLI
                stateless: None,   // Use default (true)
            };
            
            println!("{} {}", "Connecting to HTTP server:".green(), url.cyan());
            
            match manager.connect(&http_config).await {
                Ok(client) => {
                    manager.clients.insert(name.clone(), std::sync::Arc::new(client));
                    println!("{} {}", "Connected to HTTP server:".green(), name.cyan());
                    
                    // Start interactive mode with this HTTP connection
                    repl::run_with_manager(manager).await
                }
                Err(e) => {
                    eprintln!("{} {}", "Failed to connect to HTTP server:".red(), e);
                    std::process::exit(1);
                }
            }
        }
    }
}

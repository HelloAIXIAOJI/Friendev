use std::collections::HashMap;
use anyhow::Result;
use colored::Colorize;

use crate::client::ClientManager;

impl ClientManager {
    pub async fn list_tools(&self) -> Result<()> {
        if self.clients.is_empty() {
            println!("{}", "No connected servers.".yellow());
            return Ok(());
        }

        let mut tool_to_servers: HashMap<String, Vec<String>> = HashMap::new();
        let mut server_tools: Vec<(String, Vec<(String, String)>)> = Vec::new();

        for (server_name, client) in &self.clients {
            let result = client.list_tools(Default::default()).await;
            match result {
                Ok(response) => {
                    let tools: Vec<(String, String)> = response
                        .tools
                        .iter()
                        .map(|t| {
                            (
                                t.name.to_string(),
                                t.description.clone().unwrap_or_default().to_string(),
                            )
                        })
                        .collect();

                    for (tool_name, _) in &tools {
                        tool_to_servers
                            .entry(tool_name.clone())
                            .or_default()
                            .push(server_name.clone());
                    }
                    server_tools.push((server_name.clone(), tools));
                }
                Err(e) => {
                    server_tools.push((server_name.clone(), vec![]));
                    eprintln!(
                        "{} {}: {}",
                        "Error listing tools from".red(),
                        server_name,
                        e
                    );
                }
            }
        }

        let conflicts: HashMap<&String, &Vec<String>> = tool_to_servers
            .iter()
            .filter(|(_, servers)| servers.len() > 1)
            .collect();

        if !conflicts.is_empty() {
            println!(
                "{}",
                "WARNING: Tool name conflicts detected:".yellow().bold()
            );
            for (tool_name, servers) in &conflicts {
                println!(
                    "  '{}' exists in: {}",
                    tool_name.red(),
                    servers.join(", ").cyan()
                );
            }
            println!(
                "  {}",
                "Use /call server_name/tool_name to specify which server.".dimmed()
            );
            println!();
        }

        for (server_name, tools) in &server_tools {
            println!("{} {}", "Server:".bold(), server_name.cyan().bold());
            if tools.is_empty() {
                println!("  {}", "(No tools available)".dimmed());
            } else {
                for (tool_name, description) in tools {
                    if conflicts.contains_key(tool_name) {
                        println!(
                            "  {} {} {}: {}",
                            "-".dimmed(),
                            tool_name.green(),
                            "[CONFLICT]".red().bold(),
                            description.dimmed()
                        );
                    } else {
                        println!(
                            "  {} {}: {}",
                            "-".dimmed(),
                            tool_name.green(),
                            description.dimmed()
                        );
                    }
                }
            }
        }
        Ok(())
    }
}

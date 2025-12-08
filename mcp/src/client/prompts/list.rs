use std::collections::HashMap;
use anyhow::Result;
use colored::Colorize;

use crate::client::ClientManager;

impl ClientManager {
    pub async fn list_prompts(&self) -> Result<()> {
        if self.clients.is_empty() {
            println!("{}", "No connected servers.".yellow());
            return Ok(());
        }

        let mut prompt_to_servers: HashMap<String, Vec<String>> = HashMap::new();
        let mut server_prompts: Vec<(String, Vec<(String, String, Vec<String>)>)> = Vec::new();

        for (server_name, client) in &self.clients {
            let result = client.list_prompts(Default::default()).await;
            match result {
                Ok(response) => {
                    let prompts: Vec<(String, String, Vec<String>)> = response
                        .prompts
                        .iter()
                        .map(|p| {
                            let args: Vec<String> = p.arguments
                                .as_ref()
                                .unwrap_or(&vec![])
                                .iter()
                                .map(|arg| {
                                    let required = if arg.required.unwrap_or(false) {
                                        format!("{}*", arg.name)
                                    } else {
                                        arg.name.clone()
                                    };
                                    required
                                })
                                .collect();
                            (
                                p.name.clone(),
                                p.description.clone().unwrap_or_default(),
                                args,
                            )
                        })
                        .collect();

                    for (prompt_name, _, _) in &prompts {
                        prompt_to_servers
                            .entry(prompt_name.clone())
                            .or_default()
                            .push(server_name.clone());
                    }
                    server_prompts.push((server_name.clone(), prompts));
                }
                Err(e) => {
                    server_prompts.push((server_name.clone(), vec![]));
                    eprintln!(
                        "{} {}: {}",
                        "Error listing prompts from".red(),
                        server_name,
                        e
                    );
                }
            }
        }

        let conflicts: HashMap<&String, &Vec<String>> = prompt_to_servers
            .iter()
            .filter(|(_, servers)| servers.len() > 1)
            .collect();

        if !conflicts.is_empty() {
            println!(
                "{}",
                "WARNING: Prompt name conflicts detected:".yellow().bold()
            );
            for (prompt_name, servers) in &conflicts {
                println!(
                    "  '{}' exists in: {}",
                    prompt_name.red(),
                    servers.join(", ").cyan()
                );
            }
            println!(
                "  {}",
                "Use /use prompt server_name/prompt_name to specify which server.".dimmed()
            );
            println!();
        }

        for (server_name, prompts) in &server_prompts {
            println!("{} {}", "Server:".bold(), server_name.cyan().bold());
            if prompts.is_empty() {
                println!("  {}", "(No prompts available)".dimmed());
            } else {
                for (prompt_name, description, args) in prompts {
                    let conflict_marker = if conflicts.contains_key(prompt_name) {
                        format!(" {}", "[CONFLICT]".red().bold())
                    } else {
                        String::new()
                    };

                    let args_str = if args.is_empty() {
                        String::new()
                    } else {
                        format!(" ({})", args.join(", ").dimmed())
                    };

                    println!(
                        "  {} {}{}{}: {}",
                        "-".dimmed(),
                        prompt_name.green(),
                        conflict_marker,
                        args_str,
                        description.dimmed()
                    );
                }
            }
        }
        Ok(())
    }
}

use std::collections::HashMap;
use anyhow::Result;
use colored::Colorize;

use crate::client::ClientManager;

impl ClientManager {
    pub async fn list_resources(&self) -> Result<()> {
        if self.clients.is_empty() {
            println!("{}", "No connected servers.".yellow());
            return Ok(());
        }

        let mut resource_to_servers: HashMap<String, Vec<String>> = HashMap::new();
        let mut server_resources: Vec<(String, Vec<(String, String, String)>)> = Vec::new();

        for (server_name, client) in &self.clients {
            let result = client.list_resources(Default::default()).await;
            match result {
                Ok(response) => {
                    let resources: Vec<(String, String, String)> = response
                        .resources
                        .iter()
                        .map(|r| {
                            (
                                r.raw.uri.clone(),
                                r.raw.name.clone(),
                                r.raw.description.clone().unwrap_or_default(),
                            )
                        })
                        .collect();

                    for (resource_uri, _, _) in &resources {
                        resource_to_servers
                            .entry(resource_uri.clone())
                            .or_default()
                            .push(server_name.clone());
                    }
                    server_resources.push((server_name.clone(), resources));
                }
                Err(e) => {
                    server_resources.push((server_name.clone(), vec![]));
                    eprintln!(
                        "{} {}: {}",
                        "Error listing resources from".red(),
                        server_name,
                        e
                    );
                }
            }
        }

        let conflicts: HashMap<&String, &Vec<String>> = resource_to_servers
            .iter()
            .filter(|(_, servers)| servers.len() > 1)
            .collect();

        if !conflicts.is_empty() {
            println!(
                "{}",
                "WARNING: Resource URI conflicts detected:".yellow().bold()
            );
            for (resource_uri, servers) in &conflicts {
                println!(
                    "  '{}' exists in: {}",
                    resource_uri.red(),
                    servers.join(", ").cyan()
                );
            }
            println!(
                "  {}",
                "Use /read server_name/resource_uri to specify which server.".dimmed()
            );
            println!();
        }

        for (server_name, resources) in &server_resources {
            println!("{} {}", "Server:".bold(), server_name.cyan().bold());
            if resources.is_empty() {
                println!("  {}", "(No resources available)".dimmed());
            } else {
                for (resource_uri, resource_name, description) in resources {
                    if conflicts.contains_key(resource_uri) {
                        println!(
                            "  {} {} {} {}: {}",
                            "-".dimmed(),
                            resource_name.green(),
                            "[CONFLICT]".red().bold(),
                            format!("({})", resource_uri).blue(),
                            description.dimmed()
                        );
                    } else {
                        println!(
                            "  {} {} {}: {}",
                            "-".dimmed(),
                            resource_name.green(),
                            format!("({})", resource_uri).blue(),
                            description.dimmed()
                        );
                    }
                }
            }
        }
        Ok(())
    }
}

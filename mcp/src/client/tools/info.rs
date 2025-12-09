use anyhow::Result;
use colored::Colorize;

use crate::client::ClientManager;
use super::parse_tool_spec;

impl ClientManager {
    pub async fn tool_info(&self, tool_spec: &str) -> Result<()> {
        let (specified_server, tool_name) = parse_tool_spec(tool_spec);

        let mut found_tools: Vec<(String, rmcp::model::Tool)> = Vec::new();

        let servers_to_check: Vec<_> = if let Some(server_name) = specified_server {
            if let Some(client) = self.clients.get(server_name) {
                vec![(server_name.to_string(), client.clone())]
            } else {
                println!("{} '{}'", "Server not found:".yellow(), server_name);
                return Ok(());
            }
        } else {
            self.clients
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect()
        };

        for (server_name, client) in servers_to_check {
            if let Ok(response) = client.list_tools(Default::default()).await {
                for tool in response.tools {
                    if tool.name == tool_name {
                        found_tools.push((server_name.clone(), tool));
                    }
                }
            }
        }

        if found_tools.is_empty() {
            println!("{} '{}'", "Tool not found:".yellow(), tool_name);
            return Ok(());
        }

        for (server_name, tool) in &found_tools {
            print_tool_info(server_name, tool)?;
        }
        Ok(())
    }
}

fn print_tool_info(server_name: &str, tool: &rmcp::model::Tool) -> Result<()> {
    println!(
        "{}",
        format!("=== {} (from: {}) ===", tool.name, server_name)
            .cyan()
            .bold()
    );
    if let Some(title) = &tool.title {
        println!("{} {}", "Title:".bold(), title);
    }
    if let Some(desc) = &tool.description {
        println!("{} {}", "Description:".bold(), desc);
    }
    println!("{}", "Input Schema:".bold());
    println!(
        "{}",
        serde_json::to_string_pretty(&*tool.input_schema)?.dimmed()
    );
    if let Some(output_schema) = &tool.output_schema {
        println!("{}", "Output Schema:".bold());
        println!(
            "{}",
            serde_json::to_string_pretty(&**output_schema)?.dimmed()
        );
    }
    if let Some(annotations) = &tool.annotations {
        print_annotations(annotations);
    }
    println!();
    Ok(())
}

fn print_annotations(annotations: &rmcp::model::ToolAnnotations) {
    println!("{}", "Annotations:".bold());
    if let Some(read_only) = annotations.read_only_hint {
        let val = if read_only { "true".green() } else { "false".red() };
        println!("  {}: {}", "read_only".dimmed(), val);
    }
    if let Some(destructive) = annotations.destructive_hint {
        let val = if destructive { "true".red() } else { "false".green() };
        println!("  {}: {}", "destructive".dimmed(), val);
    }
    if let Some(idempotent) = annotations.idempotent_hint {
        let val = if idempotent { "true".green() } else { "false".yellow() };
        println!("  {}: {}", "idempotent".dimmed(), val);
    }
    if let Some(open_world) = annotations.open_world_hint {
        let val = if open_world { "true".yellow() } else { "false".dimmed() };
        println!("  {}: {}", "open_world".dimmed(), val);
    }
}

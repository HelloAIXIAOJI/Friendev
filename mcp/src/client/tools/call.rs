use anyhow::Result;
use colored::Colorize;
use rmcp::{model::CallToolRequestParam, service::RunningService, RoleClient};

use crate::client::ClientManager;
use super::parse_tool_spec;

impl ClientManager {
    pub async fn call_tool(&self, tool_spec: &str, args: serde_json::Value) -> Result<()> {
        let args_obj = args.as_object().cloned().unwrap_or_default();
        let (specified_server, tool_name) = parse_tool_spec(tool_spec);

        if let Some(server_name) = specified_server {
            if let Some(client) = self.clients.get(server_name) {
                return call_tool_on_server(server_name, client, tool_name, args_obj).await;
            } else {
                println!("{} '{}'", "Server not found:".yellow(), server_name);
                return Ok(());
            }
        }

        let mut servers_with_tool: Vec<String> = Vec::new();
        for (server_name, client) in &self.clients {
            if let Ok(response) = client.list_tools(Default::default()).await {
                if response.tools.iter().any(|t| t.name == tool_name) {
                    servers_with_tool.push(server_name.clone());
                }
            }
        }

        match servers_with_tool.len() {
            0 => {
                println!("{} '{}'", "Tool not found:".yellow(), tool_name);
            }
            1 => {
                let server_name = &servers_with_tool[0];
                let client = self.clients.get(server_name).unwrap();
                call_tool_on_server(server_name, client, tool_name, args_obj).await?;
            }
            _ => {
                println!(
                    "{} '{}' {}: {}",
                    "Conflict:".yellow().bold(),
                    tool_name.green(),
                    "exists in multiple servers".yellow(),
                    servers_with_tool.join(", ").cyan()
                );
                println!(
                    "{} /call {}/{} <args>",
                    "Specify server:".dimmed(),
                    "<server>".cyan(),
                    tool_name.green()
                );
            }
        }
        Ok(())
    }
}

async fn call_tool_on_server(
    server_name: &str,
    client: &RunningService<RoleClient, ()>,
    tool_name: &str,
    args_obj: serde_json::Map<String, serde_json::Value>,
) -> Result<()> {
    println!(
        "{} '{}' {} '{}'...",
        "Calling".dimmed(),
        tool_name.green(),
        "on".dimmed(),
        server_name.cyan()
    );

    let result = client
        .call_tool(CallToolRequestParam {
            name: tool_name.to_string().into(),
            arguments: Some(args_obj),
        })
        .await?;

    println!("{} {}:", "Result from".bold(), server_name.cyan());
    for content in result.content {
        println!("{}", serde_json::to_string_pretty(&content)?);
    }
    if let Some(err) = result.is_error {
        if err {
            println!("{}", "(Tool reported an error state)".red());
        }
    }
    Ok(())
}

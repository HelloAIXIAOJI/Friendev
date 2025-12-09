use anyhow::Result;
use colored::Colorize;
use base64::Engine;
use rmcp::model::{ReadResourceRequestParam, ResourceContents};

use crate::client::ClientManager;
use super::parse_resource_spec;

impl ClientManager {
    pub async fn read_resource(&self, resource_spec: &str) -> Result<()> {
        let (server_name, resource_uri) = parse_resource_spec(resource_spec);
        
        if let Some(server_name) = server_name {
            // 指定服务器
            if let Some(client) = self.clients.get(server_name) {
                return read_resource_from_server(server_name, client, resource_uri).await;
            } else {
                eprintln!("{} {}", "Server not found:".red(), server_name);
                return Ok(());
            }
        }

        // 搜索所有服务器
        let mut found = false;
        for (server_name, client) in &self.clients {
            if let Ok(response) = client.list_resources(Default::default()).await {
                if response.resources.iter().any(|r| r.raw.uri == resource_uri) {
                    if found {
                        eprintln!(
                            "{} '{}' {}",
                            "Resource".yellow(),
                            resource_uri,
                            "found in multiple servers. Use server_name/resource_uri to specify.".yellow()
                        );
                        return Ok(());
                    }
                    read_resource_from_server(server_name, client, resource_uri).await?;
                    found = true;
                }
            }
        }

        if !found {
            eprintln!("{} {}", "Resource not found:".red(), resource_uri);
        }

        Ok(())
    }
}

async fn read_resource_from_server(
    server_name: &str,
    client: &rmcp::service::RunningService<rmcp::RoleClient, ()>,
    resource_uri: &str,
) -> Result<()> {
    println!(
        "{} {} {} {}",
        "Reading resource".dimmed(),
        resource_uri.cyan(),
        "from".dimmed(),
        server_name.yellow()
    );
    
    let response = client
        .read_resource(ReadResourceRequestParam {
            uri: resource_uri.to_string(),
        })
        .await;

    match response {
        Ok(result) => {
            for content in result.contents {
                println!("{}", "━".repeat(60).dimmed());
                
                match &content {
                    ResourceContents::TextResourceContents { uri, mime_type, text, .. } => {
                        println!("{} {}", "URI:".bold(), uri.yellow());
                        if let Some(mime_type) = mime_type {
                            println!("{} {}", "MIME Type:".bold(), mime_type.cyan());
                        }
                        println!("{}", "━".repeat(60).dimmed());
                        println!("{}", text);
                    }
                    ResourceContents::BlobResourceContents { uri, mime_type, blob, .. } => {
                        println!("{} {}", "URI:".bold(), uri.yellow());
                        if let Some(mime_type) = mime_type {
                            println!("{} {}", "MIME Type:".bold(), mime_type.cyan());
                        }
                        println!("{}", "━".repeat(60).dimmed());
                        match base64::engine::general_purpose::STANDARD.decode(blob) {
                            Ok(binary_data) => {
                                println!("{} {} bytes", "Binary content:".cyan(), binary_data.len());
                                println!("{}", "Use appropriate tools to view binary content".dimmed());
                            }
                            Err(e) => {
                                eprintln!("{} {}", "Error decoding base64:".red(), e);
                            }
                        }
                    }
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("{} {}: {}", "Error reading resource".red(), resource_uri, e);
        }
    }

    Ok(())
}

use std::fs;
use std::path::Path;
use anyhow::Result;
use colored::Colorize;
use base64::Engine;
use rmcp::model::{ReadResourceRequestParam, ResourceContents};

use crate::client::ClientManager;
use super::parse_resource_spec;

impl ClientManager {
    pub async fn download_resource(&self, resource_spec: &str, local_path: &str) -> Result<()> {
        let (server_name, resource_uri) = parse_resource_spec(resource_spec);
        
        if let Some(server_name) = server_name {
            // 指定服务器
            if let Some(client) = self.clients.get(server_name) {
                return download_resource_from_server(server_name, client, resource_uri, local_path).await;
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
                    download_resource_from_server(server_name, client, resource_uri, local_path).await?;
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

async fn download_resource_from_server(
    server_name: &str,
    client: &rmcp::service::RunningService<rmcp::RoleClient, ()>,
    resource_uri: &str,
    local_path: &str,
) -> Result<()> {
    println!(
        "{} {} {} {} {} {}",
        "Downloading resource".dimmed(),
        resource_uri.cyan(),
        "from".dimmed(),
        server_name.yellow(),
        "to".dimmed(),
        local_path.green()
    );
    
    let response = client
        .read_resource(ReadResourceRequestParam {
            uri: resource_uri.to_string(),
        })
        .await;

    match response {
        Ok(result) => {
            if result.contents.is_empty() {
                eprintln!("{}", "Resource has no content".yellow());
                return Ok(());
            }

            let content = &result.contents[0];
            
            // 检查目标目录是否存在
            if let Some(parent) = Path::new(local_path).parent() {
                if !parent.exists() {
                    println!("{} {}", "Creating directory:".dimmed(), parent.display());
                    fs::create_dir_all(parent)?;
                }
            }

            match content {
                ResourceContents::TextResourceContents { text, .. } => {
                    fs::write(local_path, text)?;
                    println!("{} {} ({} characters)", 
                        "Downloaded text content to".green(), 
                        local_path.bold(), 
                        text.len()
                    );
                }
                ResourceContents::BlobResourceContents { blob, .. } => {
                    match base64::engine::general_purpose::STANDARD.decode(blob) {
                        Ok(binary_data) => {
                            let data_len = binary_data.len();
                            fs::write(local_path, binary_data)?;
                            println!("{} {} ({} bytes)", 
                                "Downloaded binary content to".green(), 
                                local_path.bold(), 
                                data_len
                            );
                        }
                        Err(e) => {
                            eprintln!("{} {}", "Error decoding base64:".red(), e);
                            return Ok(());
                        }
                    }
                }
            }

            // 显示文件信息
            if let Ok(metadata) = fs::metadata(local_path) {
                println!("{} {} bytes", "File size:".dimmed(), metadata.len());
            }
        }
        Err(e) => {
            eprintln!("{} {}: {}", "Error downloading resource".red(), resource_uri, e);
        }
    }

    Ok(())
}

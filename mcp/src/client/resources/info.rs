use anyhow::Result;
use colored::Colorize;
use rmcp::model::Resource;

use crate::client::ClientManager;
use super::parse_resource_spec;

impl ClientManager {
    pub async fn resource_info(&self, resource_spec: &str) -> Result<()> {
        let (server_name, resource_uri) = parse_resource_spec(resource_spec);
        
        if let Some(server_name) = server_name {
            // 指定服务器
            if let Some(client) = self.clients.get(server_name) {
                if let Ok(response) = client.list_resources(Default::default()).await {
                    if let Some(resource) = response.resources.iter().find(|r| r.raw.uri == resource_uri) {
                        print_resource_info(server_name, resource)?;
                    } else {
                        eprintln!("{} {} {} {}", "Resource".red(), resource_uri, "not found on server".red(), server_name);
                    }
                }
                return Ok(());
            } else {
                eprintln!("{} {}", "Server not found:".red(), server_name);
                return Ok(());
            }
        }

        // 搜索所有服务器
        let mut found = false;
        for (server_name, client) in &self.clients {
            if let Ok(response) = client.list_resources(Default::default()).await {
                if let Some(resource) = response.resources.iter().find(|r| r.raw.uri == resource_uri) {
                    if found {
                        eprintln!(
                            "{} '{}' {}",
                            "Resource".yellow(),
                            resource_uri,
                            "found in multiple servers. Use server_name/resource_uri to specify.".yellow()
                        );
                        return Ok(());
                    }
                    print_resource_info(server_name, resource)?;
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

fn print_resource_info(server_name: &str, resource: &Resource) -> Result<()> {
    println!("{}", "━".repeat(60).dimmed());
    println!("{} {}", "Server:".bold(), server_name.yellow());
    println!("{} {}", "Resource:".bold(), resource.raw.name.green());
    println!("{} {}", "URI:".bold(), resource.raw.uri.cyan());
    
    if let Some(title) = &resource.raw.title {
        println!("{} {}", "Title:".bold(), title);
    }
    
    if let Some(description) = &resource.raw.description {
        println!("{} {}", "Description:".bold(), description.dimmed());
    }
    
    if let Some(mime_type) = &resource.raw.mime_type {
        println!("{} {}", "MIME Type:".bold(), mime_type.cyan());
    }
    
    if let Some(size) = resource.raw.size {
        println!("{} {}", "Size:".bold(), format_size(size as u64));
    }
    
    if let Some(annotations) = &resource.annotations {
        println!("{}", "Annotations:".bold());
        if let Some(audience) = &annotations.audience {
            println!("  {}: {:?}", "Audience".dimmed(), audience);
        }
        if let Some(priority) = annotations.priority {
            println!("  {}: {}", "Priority".dimmed(), priority);
        }
    }
    
    println!("{}", "━".repeat(60).dimmed());
    Ok(())
}

fn format_size(size: u64) -> String {
    if size < 1024 {
        format!("{} B", size)
    } else if size < 1024 * 1024 {
        format!("{:.1} KB", size as f64 / 1024.0)
    } else if size < 1024 * 1024 * 1024 {
        format!("{:.1} MB", size as f64 / (1024.0 * 1024.0))
    } else {
        format!("{:.1} GB", size as f64 / (1024.0 * 1024.0 * 1024.0))
    }
}

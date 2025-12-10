mod config;
mod client;

pub use config::{McpConfig, McpServerConfig};
pub use client::ClientManager;

// Re-export rmcp types for commands module
pub use rmcp::model::{GetPromptRequestParam, PromptMessageContent};

use anyhow::Result;
use base64::Engine;
use colored::Colorize;
use serde_json::Value;
use std::collections::HashMap;
use i18n::I18n;

/// MCP Integration for Friendev
#[derive(Clone)]
pub struct McpIntegration {
    manager: ClientManager,
    config: McpConfig,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
    pub server: String,
}

impl McpIntegration {
    /// Initialize MCP integration
    pub async fn new() -> Result<Self> {
        let config = McpConfig::load()?;
        let mut manager = ClientManager::new();
        
        // Load and connect to all configured servers
        manager.load_from_config(&config).await?;
        
        Ok(Self { manager, config })
    }

    /// Get list of connected servers
    pub fn list_servers(&self) -> Vec<String> {
        self.manager.list_servers()
    }

    /// List all resources from available servers
    pub async fn list_resources(&self, server_filter: Option<&str>) -> Result<HashMap<String, Vec<ResourceInfo>>> {
        let mut all_resources = HashMap::new();
        
        let servers = if let Some(server) = server_filter {
            vec![server.to_string()]
        } else {
            self.list_servers()
        };

        for server in servers {
            if let Ok(client) = self.get_client(&server) {
                if let Ok(response) = client.list_resources(Default::default()).await {
                    let resources: Vec<ResourceInfo> = response.resources.into_iter()
                        .map(|res| ResourceInfo {
                            name: res.name.to_string(),
                            uri: res.uri.to_string(),
                            description: res.description.as_ref().map(|d| d.to_string()),
                            mime_type: res.mime_type.as_ref().map(|m| m.to_string()),
                        })
                        .collect();
                    all_resources.insert(server, resources);
                }
            }
        }
        
        Ok(all_resources)
    }

    /// Call a tool on a specific server
    pub async fn call_tool(&self, server: &str, tool_name: &str, args: Value) -> Result<Value> {
        use rmcp::model::CallToolRequestParam;
        
        let client = self.manager.clients.get(server)
            .ok_or_else(|| anyhow::anyhow!("MCP server '{}' not found", server))?;
        
        let params = CallToolRequestParam {
            name: tool_name.to_string().into(),
            arguments: args.as_object().cloned(),
        };
        
        let response = client.call_tool(params).await?;
        
        // Convert tool result to JSON value
        let result = if let Some(content) = response.content.first() {
            // Simply convert the content to a JSON string for now
            serde_json::to_value(content)?
        } else {
            serde_json::Value::Null
        };
        
        Ok(result)
    }

    /// Read a resource from MCP servers
    pub async fn read_resource(&self, uri: &str, server_name: Option<&str>) -> Result<String> {
        let servers = if let Some(server) = server_name {
            vec![server.to_string()]
        } else {
            self.list_servers()
        };

        for server in servers {
            match self.get_client(&server) {
                Ok(client) => {
                    match self.read_resource_from_server(client, uri).await {
                        Ok(content) => return Ok(content),
                        Err(_) => continue, // Try next server
                    }
                }
                Err(_) => continue,
            }
        }

        Err(anyhow::anyhow!("Resource '{}' not found in any connected server", uri))
    }

    /// Read a resource from a specific server client  
    async fn read_resource_from_server(&self, client: &std::sync::Arc<rmcp::service::RunningService<rmcp::RoleClient, ()>>, uri: &str) -> Result<String> {
        let params = rmcp::model::ReadResourceRequestParam {
            uri: uri.to_string(),
        };
        
        let response = client.read_resource(params).await?;
        
        // Extract text content from response
        let mut content = String::new();
        for resource_content in response.contents {
            // Try to match resource content types - just convert to JSON for now
            match serde_json::to_string_pretty(&resource_content) {
                Ok(json_str) => {
                    content.push_str(&json_str);
                    content.push('\n');
                }
                Err(_) => {
                    content.push_str("[Failed to serialize resource content]\n");
                }
            }
        }
        
        Ok(content.trim().to_string())
    }

    /// Get all available tools from all connected servers
    pub async fn get_available_tools(&self) -> Result<HashMap<String, Vec<String>>> {
        let mut server_tools = HashMap::new();
        
        for server_name in self.list_servers() {
            if let Some(client) = self.manager.clients.get(&server_name) {
                match client.list_tools(Default::default()).await {
                    Ok(response) => {
                        let tool_names: Vec<String> = response.tools.into_iter()
                            .map(|tool| tool.name.to_string())
                            .collect();
                        server_tools.insert(server_name, tool_names);
                    }
                    Err(e) => {
                        log::warn!("Failed to get tools from server '{}': {}", server_name, e);
                        server_tools.insert(server_name, vec![]);
                    }
                }
            }
        }
        
        Ok(server_tools)
    }

    /// Get tool definitions for all connected servers
    pub async fn get_server_tools_definitions(&self) -> Vec<ToolDefinition> {
        let mut all_tools = Vec::new();

        for server_name in self.list_servers() {
            if let Some(client) = self.manager.clients.get(&server_name) {
                if let Ok(response) = client.list_tools(Default::default()).await {
                    for tool in response.tools {
                        all_tools.push(ToolDefinition {
                            name: format!("{}/{}", server_name, tool.name), // Namespaced name
                            description: tool.description.unwrap_or_default().to_string(),
                            parameters: serde_json::Value::Object(tool.input_schema.as_ref().clone().into()),
                            server: server_name.clone(),
                        });
                    }
                }
            }
        }
        
        all_tools
    }

    /// Get server status with real-time information
    pub async fn get_server_status(&self) -> HashMap<String, ServerStatus> {
        let mut status = HashMap::new();
        
        for server_name in self.list_servers() {
            let mut server_status = ServerStatus {
                connected: false,
                tool_count: 0,
                resource_count: 0,
            };
            
            if let Some(client) = self.manager.clients.get(&server_name) {
                server_status.connected = true;
                
                // Get tool count
                if let Ok(tools_response) = client.list_tools(Default::default()).await {
                    server_status.tool_count = tools_response.tools.len();
                }
                
                // Get resource count
                if let Ok(resources_response) = client.list_resources(Default::default()).await {
                    server_status.resource_count = resources_response.resources.len();
                }
            }
            
            status.insert(server_name, server_status);
        }
        
        status
    }

    /// Get a client for a specific server (for commands module) 
    pub fn get_client(&self, server: &str) -> Result<&std::sync::Arc<rmcp::service::RunningService<rmcp::RoleClient, ()>>> {
        self.manager.clients.get(server)
            .ok_or_else(|| anyhow::anyhow!("MCP server '{}' not found", server))
    }
}

#[derive(Debug, Clone)]
pub struct ServerStatus {
    pub connected: bool,
    pub tool_count: usize,
    pub resource_count: usize,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ResourceInfo {
    pub name: String,
    pub uri: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

/// Display MCP status for UI (async version)
pub async fn display_mcp_status_with_i18n(integration: &McpIntegration, i18n: &I18n) {
    println!("{}", format!("🔗 {}:", i18n.get("mcp_servers")).cyan().bold());
    
    let status_map = integration.get_server_status().await;
    if status_map.is_empty() {
        println!("  {}", i18n.get("mcp_no_servers").dimmed());
        return;
    }
    
    for (name, status) in status_map {
        let status_icon = if status.connected { "✅" } else { "❌" };
        println!(
            "  {} {} ({} tools, {} resources)", 
            status_icon, 
            name.cyan(), 
            status.tool_count, 
            status.resource_count
        );
    }
}

/// Display MCP status for UI (sync version for startup)
pub fn display_mcp_status_sync_with_i18n(integration: &McpIntegration, i18n: &I18n) {
    println!("{}", format!("🔗 {}:", i18n.get("mcp_servers")).cyan().bold());
    
    let servers = integration.list_servers();
    if servers.is_empty() {
        println!("  {}", i18n.get("mcp_no_servers").dimmed());
        return;
    }
    
    for server_name in servers {
        println!("  {} {} ({})", "🔄".cyan(), server_name.cyan(), i18n.get("mcp_server_loading"));
    }
}

// Keep backward compatibility
pub async fn display_mcp_status(integration: &McpIntegration) {
    let i18n = I18n::new("enus"); // Default to English
    display_mcp_status_with_i18n(integration, &i18n).await;
}

pub fn display_mcp_status_sync(integration: &McpIntegration) {
    let i18n = I18n::new("enus"); // Default to English
    display_mcp_status_sync_with_i18n(integration, &i18n);
}

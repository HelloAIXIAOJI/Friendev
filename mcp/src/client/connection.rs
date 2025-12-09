use crate::config::McpServerConfig;
use anyhow::Result;
use colored::Colorize;
use rmcp::{
    service::{RunningService, ServiceExt},
    transport::{ConfigureCommandExt, TokioChildProcess, SseClientTransport, StreamableHttpClientTransport},
    RoleClient,
};
use std::sync::Arc;
use tokio::process::Command;

use super::ClientManager;

impl ClientManager {
    pub async fn load_from_config(&mut self, config: &crate::config::McpConfig) -> Result<()> {
        self.clients.clear();
        for (name, server_conf) in &config.mcp_servers {
            match self.connect(server_conf).await {
                Ok(client) => {
                    self.clients.insert(name.clone(), Arc::new(client));
                    println!("{} {}", "Connected:".green(), name.cyan());
                }
                Err(e) => {
                    eprintln!("{} '{}': {:#}", "Failed to connect:".red(), name, e);
                }
            }
        }
        Ok(())
    }

    pub async fn connect(&self, config: &McpServerConfig) -> Result<RunningService<RoleClient, ()>> {
        match config {
            McpServerConfig::ChildProcess { command, args, env, .. }
            | McpServerConfig::Legacy { command, args, env } => {
                let cmd = Command::new(command);
                let args = args.clone();
                let env = env.clone();

                let transport = TokioChildProcess::new(cmd.configure(move |c| {
                    c.args(&args);
                    if let Some(e) = &env {
                        c.envs(e);
                    }
                }))?;

                let client = ().serve(transport).await?;
                Ok(client)
            }
            McpServerConfig::Sse { url, auth_token, headers, .. } => {
                use rmcp::transport::sse_client::{SseClientConfig, SseClientTransport};
                
                // 🚀 OpenAI方法：在reqwest客户端层面设置default headers  
                let mut client_builder = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(30));
                
                // 如果有自定义headers，设置为default headers
                if let Some(headers) = headers {
                    let mut header_map = reqwest::header::HeaderMap::new();
                    
                    for (name, value) in headers {
                        match (name.parse::<reqwest::header::HeaderName>(), 
                               reqwest::header::HeaderValue::from_str(&value)) {
                            (Ok(header_name), Ok(header_value)) => {
                                header_map.insert(header_name, header_value);
                                eprintln!("🔍 Added SSE header: {} = {}", name, value);
                            }
                            _ => {
                                eprintln!("⚠️ Invalid SSE header: {} = {}", name, value);
                            }
                        }
                    }
                    
                    if !header_map.is_empty() {
                        client_builder = client_builder.default_headers(header_map);
                    }
                }
                
                let http_client = client_builder.build()
                    .map_err(|e| anyhow::anyhow!("Failed to build SSE client: {}", e))?;
                
                // 配置rmcp SSE transport  
                let config = SseClientConfig {
                    sse_endpoint: url.clone().try_into().unwrap(),
                    ..Default::default()
                };
                
                let transport = SseClientTransport::start_with_client(http_client, config)
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to start SSE transport: {}", e))?;
                
                let client = ().serve(transport).await?;
                Ok(client)
            }
            McpServerConfig::Http { url, auth_token, headers, stateless, .. } => {
                use rmcp::transport::streamable_http_client::{StreamableHttpClientTransportConfig, StreamableHttpClientTransport};
                
                // 🚀 OpenAI方法：在reqwest客户端层面设置default headers
                let mut client_builder = reqwest::Client::builder()
                    .timeout(std::time::Duration::from_secs(30));
                
                // 如果有自定义headers，设置为default headers
                if let Some(headers) = headers {
                    let mut header_map = reqwest::header::HeaderMap::new();
                    
                    for (name, value) in headers {
                        match (name.parse::<reqwest::header::HeaderName>(), 
                               reqwest::header::HeaderValue::from_str(&value)) {
                            (Ok(header_name), Ok(header_value)) => {
                                header_map.insert(header_name, header_value);
                                eprintln!("🔍 Added HTTP header: {} = {}", name, value);
                            }
                            _ => {
                                eprintln!("⚠️ Invalid HTTP header: {} = {}", name, value);
                            }
                        }
                    }
                    
                    if !header_map.is_empty() {
                        client_builder = client_builder.default_headers(header_map);
                    }
                }
                
                let http_client = client_builder.build()
                    .map_err(|e| anyhow::anyhow!("Failed to build HTTP client: {}", e))?;
                
                // 配置rmcp transport
                let mut config = StreamableHttpClientTransportConfig {
                    uri: url.clone().into(),
                    allow_stateless: stateless.unwrap_or(true),
                    ..Default::default()
                };
                
                // 如果有auth_token，设置为Authorization header
                if let Some(token) = auth_token {
                    config.auth_header = Some(format!("Bearer {}", token));
                }
                
                // 使用配置好headers的客户端
                let transport = StreamableHttpClientTransport::with_client(http_client, config);
                let client = ().serve(transport).await?;
                Ok(client)
            }
        }
    }
}

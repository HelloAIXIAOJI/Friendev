use std::collections::HashMap;
use anyhow::Result;
use colored::Colorize;
use rmcp::model::{GetPromptRequestParam, PromptMessageContent};

use crate::client::ClientManager;
use super::parse_prompt_spec;

impl ClientManager {
    pub async fn use_prompt(&self, prompt_spec: &str, args: &str) -> Result<()> {
        let (server_name, prompt_name) = parse_prompt_spec(prompt_spec);
        
        // 解析参数
        let prompt_args = parse_prompt_args(args);
        
        if let Some(server_name) = server_name {
            // 指定服务器
            if let Some(client) = self.clients.get(server_name) {
                return use_prompt_from_server(server_name, client, prompt_name, &prompt_args).await;
            } else {
                eprintln!("{} {}", "Server not found:".red(), server_name);
                return Ok(());
            }
        }

        // 搜索所有服务器
        let mut found = false;
        for (server_name, client) in &self.clients {
            if let Ok(response) = client.list_prompts(Default::default()).await {
                if response.prompts.iter().any(|p| p.name == prompt_name) {
                    if found {
                        eprintln!(
                            "{} '{}' {}",
                            "Prompt".yellow(),
                            prompt_name,
                            "found in multiple servers. Use server_name/prompt_name to specify.".yellow()
                        );
                        return Ok(());
                    }
                    use_prompt_from_server(server_name, client, prompt_name, &prompt_args).await?;
                    found = true;
                }
            }
        }

        if !found {
            eprintln!("{} {}", "Prompt not found:".red(), prompt_name);
        }

        Ok(())
    }
}

async fn use_prompt_from_server(
    server_name: &str,
    client: &rmcp::service::RunningService<rmcp::RoleClient, ()>,
    prompt_name: &str,
    args: &HashMap<String, String>,
) -> Result<()> {
    println!(
        "{} {} {} {}",
        "Generating prompt".dimmed(),
        prompt_name.cyan(),
        "from".dimmed(),
        server_name.yellow()
    );
    
    // 转换 HashMap 到 serde_json::Map
    let json_args: serde_json::Map<String, serde_json::Value> = args
        .iter()
        .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
        .collect();

    let response = client
        .get_prompt(GetPromptRequestParam {
            name: prompt_name.to_string(),
            arguments: Some(json_args),
        })
        .await;

    match response {
        Ok(result) => {
            // 显示描述
            if let Some(description) = &result.description {
                println!("{} {}", "✅".green(), description.green());
            }
            
            if result.messages.is_empty() {
                println!("{}", "No messages generated".yellow());
                return Ok(());
            }

            println!("{}", "━".repeat(60).dimmed());

            // 显示消息内容
            for (i, message) in result.messages.iter().enumerate() {
                let role_icon = match message.role {
                    rmcp::model::PromptMessageRole::User => "👤",
                    rmcp::model::PromptMessageRole::Assistant => "🤖",
                };
                
                let role_text = match message.role {
                    rmcp::model::PromptMessageRole::User => "User".bold(),
                    rmcp::model::PromptMessageRole::Assistant => "Assistant".bold(),
                };

                if i > 0 {
                    println!(); // 消息之间的分隔
                }
                
                println!("{} {}:", role_icon, role_text);
                
                match &message.content {
                    PromptMessageContent::Text { text } => {
                        // 简单的文本显示，保持格式
                        println!("{}", text);
                    }
                    PromptMessageContent::Image { image } => {
                        println!("{} {} ({})", 
                            "📷 Image content".cyan(),
                            format!("{} bytes", image.data.len()).dimmed(),
                            image.mime_type.dimmed()
                        );
                    }
                    PromptMessageContent::Resource { resource } => {
                        // resource 是 Annotated<RawEmbeddedResource>
                        match &resource.raw.resource {
                            rmcp::model::ResourceContents::TextResourceContents { uri, text, .. } => {
                                println!("{} {}", 
                                    "📄 Resource:".cyan(),
                                    uri.yellow()
                                );
                                println!("{}", text.dimmed());
                            }
                            rmcp::model::ResourceContents::BlobResourceContents { uri, blob, .. } => {
                                println!("{} {}", 
                                    "📄 Resource:".cyan(),
                                    uri.yellow()
                                );
                                println!("{} {} bytes", 
                                    "Binary content:".dimmed(),
                                    blob.len()
                                );
                            }
                        }
                    }
                    PromptMessageContent::ResourceLink { link } => {
                        println!("{} {}", 
                            "🔗 Resource Link:".cyan(),
                            link.uri.yellow()
                        );
                        if let Some(description) = &link.description {
                            println!("{}", description.dimmed());
                        }
                    }
                }
            }
            
            println!("{}", "━".repeat(60).dimmed());
        }
        Err(e) => {
            eprintln!("{} {}: {}", "Error generating prompt".red(), prompt_name, e);
        }
    }

    Ok(())
}

fn parse_prompt_args(args_str: &str) -> HashMap<String, String> {
    let mut args = HashMap::new();
    
    if args_str.trim().is_empty() {
        return args;
    }

    // 支持两种格式：
    // 1. key=value key2=value2
    // 2. key="quoted value" key2='single quoted'
    
    let mut chars = args_str.chars().peekable();
    let mut current_token = String::new();
    let mut tokens = Vec::new();
    let mut in_quotes = false;
    let mut quote_char = '"';

    while let Some(ch) = chars.next() {
        match ch {
            '"' | '\'' if !in_quotes => {
                in_quotes = true;
                quote_char = ch;
            }
            ch if in_quotes && ch == quote_char => {
                in_quotes = false;
            }
            ' ' | '\t' if !in_quotes => {
                if !current_token.is_empty() {
                    tokens.push(current_token.clone());
                    current_token.clear();
                }
            }
            ch => {
                current_token.push(ch);
            }
        }
    }
    
    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    // 解析 key=value 对
    for token in tokens {
        if let Some((key, value)) = token.split_once('=') {
            args.insert(key.trim().to_string(), value.trim().to_string());
        }
    }

    args
}

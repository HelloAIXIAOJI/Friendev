use anyhow::Result;
use colored::Colorize;

use crate::client::ClientManager;
use crate::config::McpConfig;

pub async fn handle_reload(manager: &mut ClientManager) -> Result<()> {
    println!("{}", "Reloading configuration...".dimmed());
    match McpConfig::load() {
        Ok(config) => {
            manager.load_from_config(&config).await?;
        }
        Err(e) => {
            eprintln!("{} {}", "Failed to load config:".red(), e);
        }
    }
    Ok(())
}

pub async fn handle_list(manager: &ClientManager, parts: &[&str]) -> Result<()> {
    if parts.len() > 1 {
        match parts[1] {
            "mcp" => {
                let servers = manager.list_servers();
                if servers.is_empty() {
                    println!("{}", "No connected servers.".yellow());
                } else {
                    for s in servers {
                        println!("{} {}", "-".dimmed(), s.cyan());
                    }
                }
            }
            "tool" => {
                if let Err(e) = manager.list_tools().await {
                    eprintln!("{} {:?}", "Error listing tools:".red(), e);
                }
            }
            "resource" => {
                if let Err(e) = manager.list_resources().await {
                    eprintln!("{} {:?}", "Error listing resources:".red(), e);
                }
            }
            "prompt" => {
                if let Err(e) = manager.list_prompts().await {
                    eprintln!("{} {:?}", "Error listing prompts:".red(), e);
                }
            }
            _ => println!("{}", "Unknown list command. Usage: /list mcp | /list tool | /list resource | /list prompt".yellow()),
        }
    } else {
        println!("{}", "Usage: /list mcp | /list tool | /list resource | /list prompt".yellow());
    }
    Ok(())
}

pub async fn handle_call(manager: &ClientManager, input: &str, parts: &[&str]) -> Result<()> {
    if parts.len() < 2 {
        println!("{}", "Usage: /call <tool_name> [json_args]".yellow());
        return Ok(());
    }
    
    let tool_name = parts[1];
    let args_start_index = input.find(tool_name).map(|i| i + tool_name.len()).unwrap_or(input.len());
    let json_str = input[args_start_index..].trim();
    
    let json_arg = if json_str.is_empty() { "{}" } else { json_str };

    match serde_json::from_str::<serde_json::Value>(json_arg) {
        Ok(args) => {
            if let Err(e) = manager.call_tool(tool_name, args).await {
                eprintln!("{} {:?}", "Error calling tool:".red(), e);
            }
        }
        Err(e) => {
            eprintln!("{} {}", "Invalid JSON arguments:".red(), e);
        }
    }
    Ok(())
}

pub async fn handle_read(manager: &ClientManager, parts: &[&str]) -> Result<()> {
    if parts.len() >= 3 && parts[1] == "resource" {
        let resource_uri = parts[2];
        if let Err(e) = manager.read_resource(resource_uri).await {
            eprintln!("{} {:?}", "Error reading resource:".red(), e);
        }
    } else {
        println!("{}", "Usage: /read resource <uri> | /read resource <server>/<uri>".yellow());
    }
    Ok(())
}

pub async fn handle_down(manager: &ClientManager, parts: &[&str]) -> Result<()> {
    if parts.len() >= 4 && parts[1] == "resource" {
        let resource_uri = parts[2];
        let local_path = parts[3];
        if let Err(e) = manager.download_resource(resource_uri, local_path).await {
            eprintln!("{} {:?}", "Error downloading resource:".red(), e);
        }
    } else {
        println!("{}", "Usage: /down resource <uri> <local_path> | /down resource <server>/<uri> <local_path>".yellow());
    }
    Ok(())
}

pub async fn handle_info(manager: &ClientManager, parts: &[&str]) -> Result<()> {
    if parts.len() >= 3 {
        match parts[1] {
            "tool" => {
                let tool_name = parts[2];
                if let Err(e) = manager.tool_info(tool_name).await {
                    eprintln!("{} {:?}", "Error getting tool info:".red(), e);
                }
            }
            "resource" => {
                let resource_uri = parts[2];
                if let Err(e) = manager.resource_info(resource_uri).await {
                    eprintln!("{} {:?}", "Error getting resource info:".red(), e);
                }
            }
            "prompt" => {
                let prompt_name = parts[2];
                if let Err(e) = manager.prompt_info(prompt_name).await {
                    eprintln!("{} {:?}", "Error getting prompt info:".red(), e);
                }
            }
            _ => println!("{}", "Usage: /info tool <tool_name> | /info resource <uri>|<server>/<uri> | /info prompt <name>|<server>/<name>".yellow()),
        }
    } else {
        println!("{}", "Usage: /info tool <tool_name> | /info resource <uri>|<server>/<uri> | /info prompt <name>|<server>/<name>".yellow());
    }
    Ok(())
}

pub fn handle_newconfig(parts: &[&str]) {
    if parts.len() < 2 {
        println!("{}", "Usage: /newconfig <name>".yellow());
    } else {
        let name = parts[1];
        match McpConfig::create_new(name) {
            Ok(path) => println!("{} {}", "Created config file:".green(), path.display()),
            Err(e) => eprintln!("{} {}", "Failed to create config:".red(), e),
        }
    }
}

pub async fn handle_use(manager: &ClientManager, parts: &[&str]) -> Result<()> {
    if parts.len() >= 3 && parts[1] == "prompt" {
        let prompt_name = parts[2];
        let args = if parts.len() > 3 {
            parts[3..].join(" ")
        } else {
            String::new()
        };
        if let Err(e) = manager.use_prompt(prompt_name, &args).await {
            eprintln!("{} {:?}", "Error using prompt:".red(), e);
        }
    } else {
        println!("{}", "Usage: /use prompt <name> [key=value...]".yellow());
    }
    Ok(())
}

pub fn handle_unknown(cmd: &str) {
    println!("{} {}", "Unknown command:".yellow(), cmd);
}

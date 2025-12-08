use anyhow::Result;
use colored::Colorize;
use rmcp::model::Prompt;

use crate::client::ClientManager;
use super::parse_prompt_spec;

impl ClientManager {
    pub async fn prompt_info(&self, prompt_spec: &str) -> Result<()> {
        let (server_name, prompt_name) = parse_prompt_spec(prompt_spec);
        
        if let Some(server_name) = server_name {
            // 指定服务器
            if let Some(client) = self.clients.get(server_name) {
                if let Ok(response) = client.list_prompts(Default::default()).await {
                    if let Some(prompt) = response.prompts.iter().find(|p| p.name == prompt_name) {
                        print_prompt_info(server_name, prompt)?;
                    } else {
                        eprintln!("{} {} {} {}", "Prompt".red(), prompt_name, "not found on server".red(), server_name);
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
            if let Ok(response) = client.list_prompts(Default::default()).await {
                if let Some(prompt) = response.prompts.iter().find(|p| p.name == prompt_name) {
                    if found {
                        eprintln!(
                            "{} '{}' {}",
                            "Prompt".yellow(),
                            prompt_name,
                            "found in multiple servers. Use server_name/prompt_name to specify.".yellow()
                        );
                        return Ok(());
                    }
                    print_prompt_info(server_name, prompt)?;
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

fn print_prompt_info(server_name: &str, prompt: &Prompt) -> Result<()> {
    println!("{}", "━".repeat(60).dimmed());
    println!("{} {}", "Server:".bold(), server_name.yellow());
    println!("{} {}", "Prompt:".bold(), prompt.name.green());
    
    if let Some(title) = &prompt.title {
        println!("{} {}", "Title:".bold(), title);
    }
    
    if let Some(description) = &prompt.description {
        println!("{} {}", "Description:".bold(), description.dimmed());
    }
    
    if let Some(ref arguments) = prompt.arguments {
        if !arguments.is_empty() {
            println!("{}", "Arguments:".bold());
            for arg in arguments {
            let required_str = if arg.required.unwrap_or(false) {
                "required".red()
            } else {
                "optional".green()
            };
            
            println!(
                "  {} {} {}",
                "•".blue(),
                arg.name.yellow(),
                format!("[{}]", required_str)
            );
            
                if let Some(description) = &arg.description {
                    println!("    {}", description.dimmed());
                }
            }
        } else {
            println!("{} {}", "Arguments:".bold(), "None".dimmed());
        }
    } else {
        println!("{} {}", "Arguments:".bold(), "None".dimmed());
    }
    
    println!();
    println!("{}", "Usage:".bold());
    let args_example = if let Some(ref arguments) = prompt.arguments {
        if arguments.is_empty() {
            String::new()
        } else {
            let examples: Vec<String> = arguments
                .iter()
                .map(|arg| format!("{}=\"example\"", arg.name))
                .collect();
            format!(" {}", examples.join(" "))
        }
    } else {
        String::new()
    };
    println!("  {}{}", format!("/use prompt {}", prompt.name).green(), args_example.dimmed());
    
    println!("{}", "━".repeat(60).dimmed());
    Ok(())
}

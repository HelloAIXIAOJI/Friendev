use anyhow::Result;
use colored::Colorize;
use mcp::{McpIntegration, GetPromptRequestParam, PromptMessageContent};
use std::io::{self, Write};
use i18n::I18n;
use config::Config;
use history::ChatSession;
use api::ApiClient;

/// Handle /prompt command - interactive MCP prompt selection and execution
pub async fn handle_prompt_command(
    mcp_integration: Option<&McpIntegration>, 
    i18n: &I18n,
    config: &mut config::Config,
    session: &mut history::ChatSession,
    api_client: &mut api::ApiClient,
) -> Result<()> {
    match mcp_integration {
        Some(integration) => {
            run_interactive_prompt_flow(integration, i18n, config, session, api_client).await
        }
        None => {
            println!("{} {}", "❌".red(), i18n.get("mcp_not_available"));
            Ok(())
        }
    }
}

async fn run_interactive_prompt_flow(
    integration: &McpIntegration, 
    i18n: &I18n, 
    config: &mut Config,
    session: &mut ChatSession,
    api_client: &mut ApiClient,
) -> Result<()> {
    println!("🎯 {}", i18n.get("prompt_interactive_flow").cyan().bold());
    println!();

    // Step 1: Select MCP server
    let server = select_mcp_server(integration, i18n).await?;
    if server.is_empty() {
        return Ok(());
    }

    // Step 2: Select prompt from the server
    let prompt_info = select_prompt(&server, integration, i18n).await?;
    let prompt_info = match prompt_info {
        Some(info) => info,
        None => return Ok(()),
    };

    // Step 3: Get prompt details and collect arguments
    let prompt_result = execute_prompt_flow(&server, &prompt_info, integration, i18n).await?;
    
    // Step 4: Display result
    display_prompt_result(&prompt_result, i18n);

    // Step 5: Send to AI for processing
    println!();
    println!("{} {}", "🚀".cyan(), i18n.get("mcp_sending_to_ai").yellow());
    send_prompt_result_to_ai(&prompt_result, config, session, api_client, i18n, Some(integration)).await?;

    Ok(())
}

async fn select_mcp_server(integration: &McpIntegration, i18n: &I18n) -> Result<String> {
    let servers = integration.list_servers();
    
    if servers.is_empty() {
        println!("{} {}", "❌".yellow(), i18n.get("mcp_no_servers_connected"));
        return Ok(String::new());
    }

    if servers.len() == 1 {
        println!("{} {}: {}", "🚀".cyan(), i18n.get("prompt_using_server"), servers[0].cyan().bold());
        return Ok(servers[0].clone());
    }

    println!("📡 {}:", i18n.get("prompt_available_servers").cyan());
    for (i, server) in servers.iter().enumerate() {
        println!("  {} {}", format!("{}.", i + 1).yellow(), server.cyan());
    }
    println!();

    loop {
        print!("{} ", i18n.get("prompt_select_server").replace("{}", "1").replace("{}", &servers.len().to_string()).green());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            return Ok(String::new());
        }

        if let Ok(choice) = input.parse::<usize>() {
            if choice > 0 && choice <= servers.len() {
                let selected = servers[choice - 1].clone();
                println!("{} {}: {}", "✅".green(), i18n.get("prompt_selected_server"), selected.cyan().bold());
                println!();
                return Ok(selected);
            }
        }

        println!("{} {} 1-{} or 'q'", "❌".red(), i18n.get("prompt_invalid_choice"), servers.len());
    }
}

async fn select_prompt(server: &str, integration: &McpIntegration, i18n: &I18n) -> Result<Option<PromptInfo>> {
    println!("{} {}...", "🔍".cyan(), i18n.get("prompt_getting_prompts").replace("{}", server));

    // Get available prompts from the server
    let prompts = match get_server_prompts(server, integration).await {
        Ok(prompts) => prompts,
        Err(e) => {
            println!("{} {}: {}", "❌".red(), i18n.get("prompt_failed_get_prompts"), e);
            return Ok(None);
        }
    };

    if prompts.is_empty() {
        println!("{} {} '{}'", "❌".yellow(), i18n.get("prompt_no_prompts"), server);
        return Ok(None);
    }

    if prompts.len() == 1 {
        let prompt = &prompts[0];
        println!("{} {}: {}", "🚀".cyan(), i18n.get("prompt_using_prompt"), prompt.name.cyan().bold());
        if let Some(desc) = &prompt.description {
            println!("  {}", desc.dimmed());
        }
        return Ok(Some(prompt.clone()));
    }

    println!("{} Available prompts:", "📝".cyan());
    for (i, prompt) in prompts.iter().enumerate() {
        print!("  {} {}", format!("{}.", i + 1).yellow(), prompt.name.cyan());
        if let Some(desc) = &prompt.description {
            print!(" - {}", desc.dimmed());
        }
        println!();
    }
    println!();

    loop {
        print!("{} ", "Select prompt (1-{}) or 'q' to quit:".green());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("q") {
            return Ok(None);
        }

        if let Ok(choice) = input.parse::<usize>() {
            if choice > 0 && choice <= prompts.len() {
                let selected = prompts[choice - 1].clone();
                println!("{} Selected: {}", "✅".green(), selected.name.cyan().bold());
                if let Some(desc) = &selected.description {
                    println!("  {}", desc.dimmed());
                }
                println!();
                return Ok(Some(selected));
            }
        }

        println!("{} Invalid choice. Please enter 1-{} or 'q'", "❌".red(), prompts.len());
    }
}

async fn get_server_prompts(server: &str, integration: &McpIntegration) -> Result<Vec<PromptInfo>> {
    // Use MCP list_prompts protocol to get actual prompts from server
    let client = integration.get_client(server)?;
    
    match client.list_prompts(Default::default()).await {
        Ok(response) => {
            let prompts: Vec<PromptInfo> = response.prompts.into_iter()
                .map(|prompt| PromptInfo {
                    name: prompt.name.to_string(),
                    description: prompt.description.as_ref().map(|d| d.to_string()),
                    arguments: prompt.arguments.unwrap_or_default().into_iter()
                        .map(|arg| PromptArgument {
                            name: arg.name.to_string(),
                            description: arg.description.as_ref().map(|d| d.to_string()),
                            required: arg.required.unwrap_or(false),
                        })
                        .collect(),
                })
                .collect();
            Ok(prompts)
        }
        Err(e) => {
            Err(anyhow::anyhow!("Failed to list prompts from server '{}': {}", server, e))
        }
    }
}

#[derive(Debug, Clone)]
struct PromptInfo {
    name: String,
    description: Option<String>,
    arguments: Vec<PromptArgument>,
}

#[derive(Debug, Clone)]  
struct PromptArgument {
    name: String,
    description: Option<String>,
    required: bool,
}

async fn execute_prompt_flow(server: &str, prompt_info: &PromptInfo, integration: &McpIntegration, i18n: &I18n) -> Result<String> {
    println!("⚡ {} '{}'", i18n.get("prompt_executing"), prompt_info.name.cyan().bold());
    
    // Collect arguments based on MCP prompt schema
    let arguments = collect_mcp_prompt_arguments(prompt_info, i18n)?;
    
    // Use MCP get_prompt to execute the prompt with collected arguments
    let client = integration.get_client(server)?;

    let prompt_params = GetPromptRequestParam {
        name: prompt_info.name.clone().into(),
        arguments: Some(arguments.as_object().cloned().unwrap_or_default()),
    };

    match client.get_prompt(prompt_params).await {
        Ok(response) => {
            // Extract content from prompt response
            let content = response.messages.into_iter()
                .map(|msg| match msg.content {
                    PromptMessageContent::Text { text } => text,
                    PromptMessageContent::Image { .. } => "[Image content]".to_string(),
                    PromptMessageContent::Resource { resource: _ } => {
                        format!("[Resource content]")
                    },
                    PromptMessageContent::ResourceLink { .. } => {
                        format!("[Resource link]")
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            Ok(content)
        }
        Err(e) => {
            let error_msg = format!("Failed to execute prompt '{}': {}", prompt_info.name, e);
            println!("{} {}", "❌".red(), error_msg);
            Ok(error_msg)
        }
    }
}

fn collect_mcp_prompt_arguments(prompt_info: &PromptInfo, i18n: &I18n) -> Result<serde_json::Value> {
    println!("📝 {} '{}'", i18n.get("prompt_collecting_args"), prompt_info.name.cyan());
    
    if prompt_info.arguments.is_empty() {
        println!("  {}", "No arguments required".dimmed());
        return Ok(serde_json::Value::Object(serde_json::Map::new()));
    }

    let mut args = serde_json::Map::new();
    
    // Collect arguments based on MCP prompt schema
    for arg in &prompt_info.arguments {
        let required_indicator = if arg.required { "*" } else { "" };
        let prompt_text = match &arg.description {
            Some(desc) => format!("{}{} ({}): ", arg.name, required_indicator, desc),
            None => format!("{}{}: ", arg.name, required_indicator),
        };

        loop {
            let value = collect_input(&prompt_text)?;
            
            // If required and empty, ask again
            if arg.required && value.trim().is_empty() {
                println!("{} This argument is required", "❌".yellow());
                continue;
            }
            
            // Store the value (empty strings are allowed for optional arguments)
            args.insert(arg.name.clone(), serde_json::Value::String(value));
            break;
        }
    }
    
    Ok(serde_json::Value::Object(args))
}

fn collect_input(prompt: &str) -> Result<String> {
    print!("{} ", prompt.green());
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn display_prompt_result(result: &str, i18n: &I18n) {
    println!();
    println!("📋 {}:", i18n.get("prompt_result_header").cyan().bold());
    println!("{}", "─".repeat(50).dimmed());
    println!();
    
    // Try to parse as JSON for better formatting
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(result) {
        match serde_json::to_string_pretty(&json_value) {
            Ok(pretty) => println!("{}", pretty),
            Err(_) => println!("{}", result),
        }
    } else {
        println!("{}", result);
    }
    
    println!();
    println!("{}", "─".repeat(50).dimmed());
    println!("{} Prompt execution completed", "✅".green());
}

/// Send prompt result to AI for processing
async fn send_prompt_result_to_ai(
    prompt_result: &str,
    _config: &mut Config,
    session: &mut ChatSession,
    api_client: &mut ApiClient,
    i18n: &I18n,
    mcp_integration: Option<&McpIntegration>,
) -> Result<()> {
    use history::Message;

    // Add the prompt result as a user message to the session
    let user_message = Message {
        role: "user".to_string(),
        content: prompt_result.to_string(),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    };
    session.add_message(user_message);

    // Build messages with current session
    let messages = session.messages.clone();

    // Send to AI and get response (AI response will be displayed by normal chat flow)
    match chat::send_and_receive(api_client, messages, session, mcp_integration, true).await {
        Ok((response_msg, tool_calls, mut displays)) => {
            // Add AI response to session (response already displayed by chat system)
            session.add_message(response_msg);

            // Handle tool calls if any
            if let Some(calls) = tool_calls {
                println!("\n{} {} {}", "🔧".cyan(), i18n.get("mcp_ai_tool_calls"), calls.len());
                
                // Execute tool calls using the existing API
                let tool_results = api::execute_tool_calls_with_mcp(
                    &calls,
                    &session.working_directory,
                    &mut displays,
                    true, // Require approval for MCP-generated tool calls
                    Some(&session.id.to_string()),
                    mcp_integration,
                    None,
                ).await;

                // Add tool results to session
                for result in tool_results {
                    session.add_message(result);
                }
            }
        }
        Err(e) => {
            println!("{} {}: {}", "❌".red(), i18n.get("mcp_ai_response_failed"), e);
            return Err(e);
        }
    }

    Ok(())
}

pub fn print_prompt_help() {
    println!("{}", "🎯 Prompt Command Help:".cyan().bold());
    println!();
    println!("  {} - Interactive MCP prompt selection and execution", "/prompt".green());
    println!();
    println!("{}", "Flow:".yellow());
    println!("  1. {} Select MCP server", "📡".cyan());
    println!("  2. {} Choose from server's available prompts", "📝".cyan()); 
    println!("  3. {} Input arguments as defined by the prompt schema", "✏️".cyan());
    println!("  4. {} Execute via MCP get_prompt and view results", "⚡".cyan());
    println!();
    println!("{}", "Features:".yellow());
    println!("  {} Fully MCP-compliant prompt discovery", "• Dynamic prompt loading from servers".dimmed());
    println!("  {} Schema-based argument collection", "• Validates required vs optional arguments".dimmed());
    println!("  {} Automatic argument validation", "• Ensures all required fields are provided".dimmed());
    println!("  {} Rich prompt descriptions and help", "• Shows prompt descriptions during selection".dimmed());
    println!();
    println!("{}", "Note:".yellow());
    println!("  All prompts and their arguments are defined by the MCP servers.");
    println!("  No hardcoded prompt types - everything is discovered dynamically.");
}

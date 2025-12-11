use anyhow::Result;
use api::ApiClient;
use history::{ChatSession, Message};

/// Optimize user prompt using AI
pub async fn optimize_prompt(
    original_prompt: &str,
    session: &ChatSession,
    api_client: &ApiClient,
) -> Result<String> {
    // Build context from recent messages
    let context = build_context(session);
    
    // Load AGENTS.md if available (from session's working directory)
    let agents_context = load_agents_context(&session.working_directory);
    
    // Create optimization request
    let system_message = create_optimization_system_prompt(&agents_context);
    let user_message = create_optimization_request(original_prompt, &context);
    
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: system_message,
            tool_calls: None,
            tool_call_id: None,
            name: None,
        },
        Message {
            role: "user".to_string(),
            content: user_message,
            tool_calls: None,
            tool_call_id: None,
            name: None,
        },
    ];
    
    // Call API (non-streaming for simplicity)
    let response = api_client.chat_complete(messages, None).await?;
    
    Ok(response.content.trim().to_string())
}

/// Build context from recent conversation
fn build_context(session: &ChatSession) -> String {
    let messages = &session.messages;
    
    // Get last 3 message pairs (user + assistant)
    let recent_messages: Vec<String> = messages
        .iter()
        .rev()
        .take(6)
        .rev()
        .map(|msg| {
            let role = if msg.role == "user" { "User" } else { "Assistant" };
            format!("{}: {}", role, msg.content.chars().take(150).collect::<String>())
        })
        .collect();
    
    if recent_messages.is_empty() {
        String::from("No previous conversation")
    } else {
        recent_messages.join("\n")
    }
}

/// Load AGENTS.md context if available
fn load_agents_context(working_dir: &std::path::Path) -> String {
    match agents::load_agents_md(working_dir) {
        Ok(Some(content)) => content,
        _ => String::new(),
    }
}

/// Create system prompt for optimization
fn create_optimization_system_prompt(agents_context: &str) -> String {
    // Load the optimizer prompt template
    let template = include_str!("../../../prompts/src/optimizer_prompt.md");
    
    // Format agents context section
    let agents_section = if !agents_context.is_empty() {
        format!(
            r#"
## Available Project Context

You have access to the project's AGENTS.md file, which contains:
- Project structure and architecture
- Development environment setup
- Build and compilation instructions
- Code style conventions
- Dependencies and tools used

**Use this context to**:
- Suggest appropriate programming languages and frameworks already in use
- Reference existing project structure when relevant
- Align with established code conventions
- Propose solutions that fit the project's architecture

**Key Project Information**:
```
{}
```
"#,
            agents_context.chars().take(2000).collect::<String>()
        )
    } else {
        String::from("\n## Available Project Context\n\nNo AGENTS.md found. Optimize based on general best practices.\n")
    };
    
    // Replace the {agents_context} placeholder
    template.replace("{agents_context}", &agents_section)
}

/// Create optimization request with context
fn create_optimization_request(original: &str, context: &str) -> String {
    format!(
        r#"# Recent Conversation Context
```
{}
```

# User's Current Input
```
{}
```

Transform the user's input above into a clear, detailed, and well-structured prompt. Reference the conversation context if relevant to provide continuity."#,
        context, original
    )
}

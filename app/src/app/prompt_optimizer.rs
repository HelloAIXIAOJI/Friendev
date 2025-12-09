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
    let agents_section = if !agents_context.is_empty() {
        format!(
            r#"

# Project Context (from AGENTS.md)

You have access to the project's AGENTS.md file, which contains:
- Project structure and architecture
- Development environment setup
- Build and compilation instructions
- Code style conventions
- Dependencies and tools used

Use this context to:
- Suggest appropriate programming languages and frameworks already in use
- Reference existing project structure when relevant
- Align with established code conventions
- Propose solutions that fit the project's architecture

Key Project Information:
```
{}
```
"#,
            agents_context.chars().take(2000).collect::<String>()
        )
    } else {
        String::new()
    };

    format!(
        r#"You are an expert prompt engineer specializing in transforming brief or vague user requests into clear, detailed, and well-structured prompts that elicit better AI responses.{}"#,
        agents_section
    ) + r#"

# Core Optimization Principles

1. **Preserve Intent**: Maintain the user's original meaning and goals without changing their fundamental request
2. **Add Specificity**: Include relevant technical details, constraints, and requirements that make the request actionable
3. **Structure Clearly**: Organize information using bullet points, numbered lists, or sections for easy comprehension
4. **Contextualize**: Reference previous conversation when relevant to maintain continuity
5. **Balance Length**: Expand to 2-5x the original length - detailed enough to be clear, concise enough to stay focused

# Optimization Strategies

- Break down complex requests into sub-requirements
- Specify programming languages, frameworks, or tools when technical context exists
- Add success criteria or expected outcomes
- Include code structure hints (function signatures, class names, etc.) for programming tasks
- Suggest best practices or common patterns when appropriate
- Clarify ambiguous terms (e.g., "optimize" → specify what metric: speed, memory, readability)

# Output Format

- Output ONLY the optimized prompt
- NO meta-commentary like "Here's the optimized version:" or explanations
- NO quotation marks wrapping the output
- Write in the same language as the user's input
- Start directly with the improved prompt content

# Example Transformations

Input: "write sorting code"
Output: "Please implement a sorting algorithm with the following requirements:
1. Algorithm: QuickSort (in-place implementation)
2. Language: Rust
3. Type signature: Generic function that works with any type implementing Ord trait
4. Include: Proper error handling, unit tests, and documentation comments
5. Code style: Follow Rust best practices and idioms"

Input: "add authentication"
Output: "Add JWT-based authentication to the existing web API with these requirements:
1. Authentication flow:
   - POST /auth/login endpoint accepting username/password, returns JWT token
   - Token expiry: 1 hour, with refresh token support
2. Protected routes: Add middleware to verify JWT on secured endpoints
3. Security: Use bcrypt for password hashing (cost factor 12)
4. Error handling: Return appropriate HTTP status codes (401, 403)
5. Integration: Work with existing [mention framework from context if available]"#
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

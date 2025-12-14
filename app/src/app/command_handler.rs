use super::notification;
use super::startup::AppState;
use anyhow::Result;
use api;
use chat;
use commands;
use history::Message;
use security;
use ui::get_i18n;
use futures::future::BoxFuture;
use tools::{HookType, execute_hook, HookContext};

/// Handle user input and command processing
pub async fn handle_user_input(line: &str, state: &mut AppState) -> Result<()> {
    // Handle commands
    if line.starts_with('/') {
        // Pre-Command Hook
        let hook_ctx = HookContext::new(state.session.working_directory.clone())
            .with_env("FRIENDEV_COMMAND", line);
        
        // We need to pass a callback to execute_hook.
        // However, we have &mut state here.
        // If we pass a closure that uses state, we might have borrowing issues if execute_hook is async and held across await points?
        // But execute_hook is awaited immediately.
        // The callback: Fn(&str) -> Future.
        // Since we are in `handle_user_input`, we can't easily clone `state` or share mutable access.
        // This is the tricky part.
        // If the hook wants to run a command, that command execution (handle_command) requires `&mut state`.
        // But we are currently holding `&mut state` in `handle_user_input`.
        // So we can't pass a closure that borrows `state` mutably if `execute_hook` takes ownership or if rustc can't prove disjointness.
        // Actually, we can reborrow state inside the closure.
        // BUT `execute_hook` is defined in `tools`, so it doesn't know about `AppState`.
        // The callback signature is `Fn(&str) -> BoxFuture<Result<()>>`.
        // The closure must capture `state`. Since `state` is `&mut`, the closure can capture it.
        // However, `Fn` implies callable multiple times. `&mut` cannot be moved into a `Fn` closure if it's called multiple times concurrently? No, it's sequential.
        // But `Fn` requires shared reference `&self`, so it can't mutate captured `&mut state` unless it's `FnMut`.
        // Our `CommandRunner` type is `Fn`, not `FnMut`.
        // If we change `CommandRunner` to `FnMut`, we might be able to do it.
        // But `execute_hook` takes `&self` or `&mut self`? It takes `command_runner: Option<&FriendevCommandRunner>`.
        // `FriendevCommandRunner` is `dyn Fn`.
        
        // Limitation: We cannot easily support executing Friendev commands that modify state inside hooks due to borrowing rules
        // without significant refactoring (e.g. interior mutability).
        // For now, let's pass None to avoid fighting the borrow checker, 
        // unless the user specifically requested "recursive" command execution which is dangerous anyway.
        // If the user really needs it, we'd need `RefCell` or `Arc<Mutex>` for AppState.
        
        // Wait, `state` is `&mut AppState`.
        // If we want to support `/index` command which modifies session/config, we need mutable access.
        // Let's see if we can clone the necessary parts for *read-only* commands?
        // Or just support commands that don't require full state?
        // Actually, `handle_command` requires mutable access.
        
        // DECISION: For this iteration, we will pass `None` in `handle_user_input` as well, 
        // because enabling re-entrant command execution is architecturally complex in Rust without shared ownership.
        // The user's request "/index outline all" suggests they want to run tools.
        // Tools run via `api::execute_tool_calls` or `commands`.
        
        // Alternative: If we really want to support this, we might need to use `Arc<Mutex<AppState>>` eventually.
        // For now, let's try to implement it only if we can clone what's needed. 
        // `ApiClient` is cloneable. `Config` is cloneable.
        // `ChatSession` is cloneable (expensive?).
        // If we clone them, we operate on a *copy* of the state. 
        // For `/index`, it affects the filesystem/database, so it works fine even with cloned state!
        // For `/model`, it affects Config. If we operate on a copy, the main config won't update.
        // This means hooks can run *side-effect* commands (like indexing), but not state-modifying commands (like switching model for the main session).
        // This seems like a reasonable compromise.
        
        let mut client_clone = state.api_client.clone();
        let mut config_clone = state.config.clone();
        let mut session_clone = state.session.clone();
        
        // We can use a move closure that owns these clones.
        // But `FriendevCommandRunner` is `Fn`. The closure will be called multiple times?
        // `execute_hook` iterates over steps. Yes.
        // So the closure must be `Fn`. It can access the clones via reference.
        
        /* 
        let runner = move |cmd: &str| -> BoxFuture<'static, Result<()>> {
            let cmd_string = cmd.to_string();
            let mut my_config = config_clone.clone();
            let mut my_session = session_clone.clone();
            let mut my_client = client_clone.clone();
            
            Box::pin(async move {
                commands::handle_command(&cmd_string, &mut my_config, &mut my_session, &mut my_client).await
            })
        };
        
        if let Err(e) = execute_hook(HookType::PreCommand, &hook_ctx, Some(&runner)).await {
             eprintln!("\n\x1b[33m[!] PreCommand Hook Error: {}\x1b[0m\n", e);
        }
        */
        
        // However, `commands::handle_command` is not pub visible from here? `commands` crate is dependency.
        // `app` depends on `commands`. `commands` exposes `handle_command`.
        
        // Let's try to implement this logic.
        // Note: `BoxFuture` needs import.
        
        let runner_client = state.api_client.clone();
        let runner_config = state.config.clone();
        let runner_session = state.session.clone();
        
        let runner = move |cmd: &str| -> BoxFuture<'static, Result<()>> {
            let cmd_string = cmd.to_string();
            let mut my_config = runner_config.clone();
            let mut my_session = runner_session.clone();
            let mut my_client = runner_client.clone();
            
            Box::pin(async move {
                commands::handle_command(&cmd_string, &mut my_config, &mut my_session, &mut my_client).await
            })
        };

        if let Err(e) = execute_hook(HookType::PreCommand, &hook_ctx, Some(&runner)).await {
             eprintln!("\n\x1b[33m[!] PreCommand Hook Error: {}\x1b[0m\n", e);
        }

        // Special handling for /agents.md command
        if line == "/agents.md" {
            handle_agents_md_command(state).await?;
        } else if line == "/send.md" {
            handle_send_file_command(state).await?;
        } else {
            // Other commands
            if let Err(e) = commands::handle_command_with_mcp(
                line,
                &mut state.config,
                &mut state.session,
                &mut state.api_client,
                state.mcp_integration.as_ref(),
            )
            .await
            {
                let i18n = get_i18n();
                eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), e);
            }
        }

        // Post-Command Hook
        if let Err(e) = execute_hook(HookType::PostCommand, &hook_ctx, Some(&runner)).await {
             eprintln!("\n\x1b[33m[!] PostCommand Hook Error: {}\x1b[0m\n", e);
        }

        return Ok(());
    }

    // Security check: intercept suspicious input
    if security::is_input_suspicious(line) {
        let i18n = get_i18n();
        eprintln!(
            "\n\x1b[31m[X] {}:\x1b[0m {}\n",
            i18n.get("security_warning_label"),
            i18n.get("security_forbidden_tokens")
        );
        return Ok(());
    }

    // User message
    let user_message = Message {
        role: "user".to_string(),
        content: line.to_string(),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    };
    state.session.add_message(user_message);

    // Process chat and tool calls
    if chat::run_agent_loop(
        &state.api_client,
        &state.config,
        &mut state.session,
        state.mcp_integration.as_ref(),
        state.auto_approve,
        None,
    )
    .await?
    {
        // Send notification asynchronously without blocking
        tokio::spawn(async move {
            let _ = notification::notify_ai_completed().await;
        });
    }

    state.session.save()?;
    Ok(())
}

/// Handle /agents.md command
async fn handle_agents_md_command(state: &mut AppState) -> Result<()> {
    match commands::handle_agents_md_command(&state.session, &state.i18n).await {
        Ok(analysis_prompt) => {
            // Add prompt as USER message to session
            let analysis_message = Message {
                role: "user".to_string(),
                content: analysis_prompt,
                tool_calls: None,
                tool_call_id: None,
                name: None,
            };
            state.session.add_message(analysis_message);

            // Auto-send to AI (same flow as normal user message)
            if chat::run_agent_loop(
                &state.api_client,
                &state.config,
                &mut state.session,
                state.mcp_integration.as_ref(),
                state.auto_approve,
                None,
            )
            .await?
            {
                // Send notification asynchronously without blocking
                tokio::spawn(async move {
                    let _ = notification::notify_ai_completed().await;
                });
            }
            state.session.save()?;
        }
        Err(e) => eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", state.i18n.get("error"), e),
    }
    Ok(())
}

/// Handle the /send.md command to read and send send.md file content to AI
async fn handle_send_file_command(state: &mut AppState) -> Result<()> {
    let i18n = get_i18n();
    
    // Construct the path to send.md in the current working directory
    let send_file_path = std::path::Path::new(&state.session.working_directory).join("send.md");
    
    // Check if path exists and is actually a file (not a directory)
    match send_file_path.metadata() {
        Ok(metadata) => {
            if !metadata.is_file() {
                eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), i18n.get("send_md_is_dir"));
                return Ok(());
            }
        }
        Err(_) => {
            eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), i18n.get("send_md_not_found"));
            return Ok(());
        }
    }
    
    // Read file content
    let content = match std::fs::read_to_string(&send_file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), i18n.get("send_md_read_error").replace("{}", &e.to_string()));
            return Ok(());
        }
    };
    
    // Print confirmation
    println!("\x1b[36m{}\x1b[0m", i18n.get("send_md_sending"));
    
    // Create message with file content
    let file_message = Message {
        role: "user".to_string(),
        content,
        tool_calls: None,
        tool_call_id: None,
        name: None,
    };
    
    // Add message to session
    state.session.add_message(file_message);
    
    // Process chat and tool calls
    if chat::run_agent_loop(
        &state.api_client,
        &state.config,
        &mut state.session,
        state.mcp_integration.as_ref(),
        state.auto_approve,
        None,
    )
    .await?
    {
        // Send notification asynchronously without blocking
        tokio::spawn(async move {
            let _ = notification::notify_ai_completed().await;
        });
    }
    state.session.save()?;
    
    // Ensure prompt appears immediately
    use std::io::Write;
    std::io::Write::flush(&mut std::io::stdout()).ok();
    
    Ok(())
}
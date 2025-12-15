use super::output_formatter;
use anyhow::Result;
use api::ToolCallAccumulator;
use history::{ChatSession, Message};
use std::collections::HashMap;
use ui::ToolCallDisplay;
use crossterm::event::{poll, read, Event, KeyCode};
use std::time::Duration;

/// Send messages to AI and receive response
/// 
/// # Parameters
/// - `is_first_turn`: Whether this is the first turn (not a tool call loop iteration)
/// - `force_no_animation`: Force to not show spinning animation even for first turn
pub async fn send_and_receive(
    client: &api::ApiClient,
    messages: Vec<Message>,
    _session: &ChatSession,
    mcp_integration: Option<&mcp::McpIntegration>,
    is_first_turn: bool,
    force_no_animation: bool,
) -> Result<(
    Message,
    Option<Vec<history::ToolCall>>,
    HashMap<String, ToolCallDisplay>,
)> {
    // Use non-streaming request with retry and animation
    // Check for interrupt before starting
    if check_interrupt()? {
        let i18n = ui::get_i18n();
        println!("\n\n\x1b[33m⚠ {}\x1b[0m\n", i18n.get("hint_esc"));
        let message = Message {
            role: "assistant".to_string(),
            content: i18n.get("hint_esc"),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        };
        return Ok((message, None, HashMap::new()));
    }
    
    // Show streaming animation for first turn AI requests
    let spinner = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
    let start_time = std::time::Instant::now();
    
    // Print AI prefix on first turn
    if is_first_turn {
        output_formatter::print_ai_prefix();
    }
    
    // Spawn streaming spinner if enabled
    let tx = if is_first_turn && !force_no_animation {
        let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(1);
        let _spinner_handle = tokio::spawn(async move {
            let mut i = 0;
            loop {
                tokio::select! {
                    _ = tokio::time::sleep(std::time::Duration::from_millis(200)) => {
                        let elapsed = start_time.elapsed().as_secs();
                        // Update the animation line in place
                        print!("\r\x1b[36m[Streaming {} [{}s]\x1b[0m", spinner[i % spinner.len()], elapsed);
                        std::io::Write::flush(&mut std::io::stdout()).ok();
                        i += 1;
                    }
                    _ = rx.recv() => {
                        // Clear the animation line before exiting
                        print!("\r\x1b[K");
                        std::io::Write::flush(&mut std::io::stdout()).ok();
                        break;
                    }
                }
            }
        });
        Some(tx)
    } else {
        None
    };
    
    // Make API request
    let response = client.chat_with_retry(messages, mcp_integration).await?;
    
    // Stop streaming animation
    if let Some(tx) = tx {
        // Clear the animation line first
        print!("\r\x1b[K");
        std::io::Write::flush(&mut std::io::stdout()).ok();
        
        // Send stop signal
        let _ = tx.send(()).await;
        
        // Small delay to ensure animation task can respond
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    
    // Handle content and reasoning like the streaming version
    let mut has_reasoning = false; // Non-streaming doesn't have reasoning
    let content_empty = response.content.is_empty();
    
    // Print the response content
    if !response.content.is_empty() {
        output_formatter::print_content(&response.content, &mut has_reasoning)?;
    }
    
    // Get tool calls from response and use ToolCallAccumulator to handle them properly
    let tool_calls = response.tool_calls;
    let mut tool_accumulator = ToolCallAccumulator::new();
    
    // Process tool calls through the accumulator like the streaming version
    if let Some(ref calls) = tool_calls {
        if !calls.is_empty() {
            // Print tool call separator like streaming version
            output_formatter::print_tool_call_separator()?;
            
            // Use the existing UI system to show tool calls
            for call in calls {
                tool_accumulator.add_chunk(
                    call.id.clone(),
                    call.function.name.clone(), 
                    call.function.arguments.clone()
                );
            }
        }
    }

    // Get displays before consuming the accumulator
    let mut displays_map = tool_accumulator.get_displays().clone();
    
    // Trigger streaming displays for all tool calls
    for display in displays_map.values_mut() {
        display.render_streaming();
    }

    // Finalize output like streaming version
    output_formatter::finalize_output(has_reasoning, content_empty)?;

    let final_tool_calls = tool_accumulator.into_tool_calls();
    
    // Convert displays to HashMap<String, ToolCallDisplay>
    let displays: HashMap<String, ToolCallDisplay> = displays_map
        .into_iter()
        .collect();
    
    let message = Message {
        role: "assistant".to_string(),
        content: response.content,
        tool_calls: if final_tool_calls.is_empty() { None } else { Some(final_tool_calls.clone()) },
        tool_call_id: None,
        name: None,
    };

    Ok((message, if final_tool_calls.is_empty() { None } else { Some(final_tool_calls) }, displays))
}

/// Check if ESC key is pressed (non-blocking)
fn check_interrupt() -> Result<bool> {
    // Poll with a slightly longer timeout for better key detection
    // 10ms is still fast enough to feel responsive but more reliable
    if poll(Duration::from_millis(10))? {
        if let Event::Key(key_event) = read()? {
            // Check for ESC key
            if key_event.code == KeyCode::Esc {
                return Ok(true);
            }
            // Also check for Ctrl+C as alternative interrupt
            if key_event.code == KeyCode::Char('c') 
                && key_event.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

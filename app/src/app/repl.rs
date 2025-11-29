use super::command_handler;
use super::prompt_optimizer;
use super::reedline_config::{create_prompt, create_reedline, process_signal, InputResult};
use super::startup::AppState;
use anyhow::Result;
use reedline::{EditCommand, Reedline};
use std::time::{Duration, Instant};
use ui::get_i18n;

/// Run the REPL loop with reedline
pub async fn run_repl(mut state: AppState) -> Result<()> {
    let mut line_editor = create_reedline()?;
    let prompt = create_prompt();
    let mut last_ctrl_c: Option<Instant> = None;

    loop {
        let sig = line_editor.read_line(&prompt);

        match sig {
            Ok(signal) => match process_signal(signal) {
                InputResult::Input(buffer) => {
                    // Reset Ctrl+C counter on successful input
                    last_ctrl_c = None;
                    
                    if buffer.is_empty() {
                        continue;
                    }

                    // Handle user input and commands
                    if let Err(e) = command_handler::handle_user_input(&buffer, &mut state).await {
                        let i18n = get_i18n();
                        eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), e);
                    }
                }
                InputResult::OptimizePrompt(original) => {
                    // Reset Ctrl+C counter
                    last_ctrl_c = None;
                    
                    if original.trim().is_empty() {
                        continue;
                    }
                    
                    // Optimize the prompt
                    println!("\n\x1b[36m⚙ 正在优化提示词...\x1b[0m");
                    match prompt_optimizer::optimize_prompt(&original, &state.session, &state.api_client).await {
                        Ok(optimized) => {
                            // Pre-fill the input with optimized text
                            if let Err(e) = prefill_input(&mut line_editor, &optimized) {
                                let i18n = get_i18n();
                                eprintln!("\x1b[33m[!] {}:\x1b[0m {}\n", i18n.get("error"), e);
                            }
                        }
                        Err(e) => {
                            let i18n = get_i18n();
                            eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), e);
                        }
                    }
                }
                InputResult::CtrlC => {
                    let i18n = get_i18n();
                    let now = Instant::now();
                    
                    // Check if this is the second Ctrl+C within 2 seconds
                    if let Some(last_time) = last_ctrl_c {
                        if now.duration_since(last_time) < Duration::from_secs(2) {
                            println!("\n\x1b[36m{}\x1b[0m\n", i18n.get("goodbye"));
                            break;
                        }
                    }
                    
                    // First Ctrl+C or timeout expired
                    last_ctrl_c = Some(now);
                    println!("\n\x1b[33m^C\x1b[0m \x1b[90m({})\x1b[0m", i18n.get("hint_ctrl_c_twice"));
                    continue;
                }
                InputResult::CtrlD => {
                    let i18n = get_i18n();
                    println!("\n\x1b[36m{}\x1b[0m\n", i18n.get("goodbye"));
                    break;
                }
                InputResult::Error(err) => {
                    let i18n = get_i18n();
                    eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), err);
                }
            },
            Err(err) => {
                let i18n = get_i18n();
                eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), err);
                break;
            }
        }
    }

    Ok(())
}

/// Pre-fill the input buffer with text
fn prefill_input(line_editor: &mut Reedline, text: &str) -> Result<()> {
    // If text is multi-line, print dynamic newlines to force scroll and ensure space.
    // We calculate the number of lines and add 5 extra for buffer.
    let newline_count = text.matches('\n').count();
    if newline_count > 0 {
        print!("{}", "\n".repeat(newline_count + 5));
    }

    // Use the run_edit_commands API to insert text
    line_editor.run_edit_commands(&[
        EditCommand::Clear,
        EditCommand::InsertString(text.to_string()),
        EditCommand::MoveToStart, // Ensure cursor is at the start
    ]);
    
    Ok(())
}

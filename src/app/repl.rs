use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use super::startup::AppState;
use super::command_handler;

/// Run the REPL loop
pub async fn run_repl(mut state: AppState) -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    
    loop {
        let readline = rl.readline(">> ");
        
        match readline {
            Ok(line) => {
                let line = line.trim();
                
                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line)?;

                // Handle user input and commands
                if let Err(e) = command_handler::handle_user_input(line, &mut state).await {
                    eprintln!("\n\x1b[31m[X] Error:\x1b[0m {}\n", e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\n\x1b[33m^C\x1b[0m");
                continue;
            }
            Err(ReadlineError::Eof) => {
                println!("\n\x1b[36mGoodbye!\x1b[0m\n");
                break;
            }
            Err(err) => {
                eprintln!("\n\x1b[31m[X] Error:\x1b[0m {}\n", err);
                break;
            }
        }
    }

    Ok(())
}

use super::command_handler;
use super::startup::AppState;
use crate::ui::get_i18n;
use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

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
                    let i18n = get_i18n();
                    eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), e);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("\n\x1b[33m^C\x1b[0m"); // 保持 Ctrl+C 标记不做 i18n
                continue;
            }
            Err(ReadlineError::Eof) => {
                let i18n = get_i18n();
                println!("\n\x1b[36m{}\x1b[0m\n", i18n.get("goodbye"));
                break;
            }
            Err(err) => {
                let i18n = get_i18n();
                eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", i18n.get("error"), err);
                break;
            }
        }
    }

    Ok(())
}

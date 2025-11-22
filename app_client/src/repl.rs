use anyhow::Result;
use rpc::AppService;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

/// Run the REPL loop
pub async fn run_repl(mut service: Box<dyn AppService>) -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        drain_events(service.as_mut()).await;

        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                let line = line.trim();

                if line.is_empty() {
                    continue;
                }

                rl.add_history_entry(line)?;

                if let Err(e) = service.handle_user_input(line).await {
                    let label = service
                        .get_message("error")
                        .await
                        .unwrap_or_else(|_| "Error".to_string());
                    eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", label, e);
                }

                drain_events(service.as_mut()).await;
            }
            Err(ReadlineError::Interrupted) => {
                println!("\n\x1b[33m^C\x1b[0m");
                continue;
            }
            Err(ReadlineError::Eof) => {
                let goodbye = service
                    .get_message("goodbye")
                    .await
                    .unwrap_or_else(|_| "Goodbye".to_string());
                println!("\n\x1b[36m{}\x1b[0m\n", goodbye);
                break;
            }
            Err(err) => {
                let label = service
                    .get_message("error")
                    .await
                    .unwrap_or_else(|_| "Error".to_string());
                eprintln!("\n\x1b[31m[X] {}:\x1b[0m {}\n", label, err);
                break;
            }
        }
    }

    Ok(())
}

async fn drain_events(service: &mut dyn AppService) {
    while let Some(event) = service.next_event().await {
        handle_event(event);
    }
}

fn handle_event(event: rpc::protocol::StreamEvent) {
    match event {
        rpc::protocol::StreamEvent::OutputLine(line) => println!("{}", line),
        rpc::protocol::StreamEvent::ToolStatus { id, status } => {
            println!("[tool:{}] {}", id, status)
        }
    }
}

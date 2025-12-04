use super::review;
use anyhow::Result;
use api::ApiClient;
use config::Config;
use history::ChatSession;
use i18n::I18n;
use prompts;
use std::env;
use ui;
use futures::future::BoxFuture;
use commands;

/// Application startup state
pub struct AppState {
    pub config: Config,
    pub i18n: I18n,
    pub session: ChatSession,
    pub api_client: ApiClient,
    pub auto_approve: bool,
}

/// Initialize the application
pub async fn initialize_app() -> Result<AppState> {
    // Check for smart approval flags
    let smart_approve = env::args().any(|arg| 
        arg == "--shorekeeper" || 
        arg == "--ally-but-i-dont-fully-trust" || 
        arg == "--ew"
    );

    // Check for jury mode flag
    let jury_mode = env::args().any(|arg| arg == "--jury");

    if smart_approve {
        ui::set_smart_approval_mode(true);
    } else if jury_mode {
        ui::set_jury_mode(true);
    }

    // Check for --ally or --yolo flag (disabled if smart approval or jury is active)
    let auto_approve = !smart_approve && !jury_mode && env::args().any(|arg| arg == "--ally" || arg == "--yolo");

    // Check for --setup flag to force setup
    let force_setup = env::args().any(|arg| arg == "--setup");

    // Load or initialize config
    let config = if force_setup {
        // Force setup regardless of existing config
        Config::initialize()?
    } else {
        match Config::load()? {
            Some(c) => c,
            None => Config::initialize()?,
        }
    };

    // Create i18n instance
    let i18n = I18n::new(&config.ui_language);

    println!(
        "\x1b[32m[OK]\x1b[0m \x1b[2m{}\x1b[0m\n",
        i18n.get("config_loaded")
    );

    // Clean up empty sessions
    ChatSession::cleanup_empty_sessions()?;

    // Get current working directory
    let working_dir = env::current_dir()?;
    println!(
        "\x1b[36m[DIR]\x1b[0m \x1b[2m{}\x1b[0m\n",
        working_dir.display()
    );

    // Create or load chat session
    let session = ChatSession::new(working_dir.clone());
    session.save()?;
    println!(
        "\x1b[32m[OK]\x1b[0m \x1b[2m{}:\x1b[0m \x1b[90m{}\x1b[0m\n",
        i18n.get("new_session"),
        session.id
    );

    // Create API client
    let api_client = ApiClient::new(config.clone());

    // Install review handler for approval prompts
    review::install_review_handler(api_client.clone(), config.clone());

    // Execute Startup Hook
    use tools::{HookType, execute_hook, HookContext};
    let hook_ctx = HookContext::new(working_dir.clone());
    
    // Create command runner for startup hooks
    let runner_client = api_client.clone();
    let runner_config = config.clone();
    let runner_session = session.clone();
    
    let runner = move |cmd: &str| -> BoxFuture<'static, Result<()>> {
        let cmd_string = cmd.to_string();
        let mut my_config = runner_config.clone();
        let mut my_session = runner_session.clone();
        let mut my_client = runner_client.clone();
        
        Box::pin(async move {
            commands::handle_command(&cmd_string, &mut my_config, &mut my_session, &mut my_client).await
        })
    };

    if let Err(e) = execute_hook(HookType::Startup, &hook_ctx, Some(&runner)).await {
        eprintln!("\n\x1b[33m[!] Startup Hook Error: {}\x1b[0m\n", e);
    }

    // Check outline index freshness
    check_outline_freshness(&working_dir, &i18n);

    // Print welcome message
    prompts::print_welcome(&config, &i18n);

    Ok(AppState {
        config,
        i18n,
        session,
        api_client,
        auto_approve,
    })
}

fn check_outline_freshness(working_dir: &std::path::Path, i18n: &I18n) {
    // Simple check: if .friendev/index/outline.db exists, check git commits.
    // If not exists or > 15 commits diff, warn user.
    use tools::tools::indexer::Indexer;
    use std::process::Command;
    use colored::Colorize;

    if let Ok(indexer) = Indexer::new(working_dir) {
        if let Ok(Some(last_hash)) = indexer.get_last_commit() {
            // Check git log count
            let output = Command::new("git")
                .args(["rev-list", "--count", "HEAD", &format!("^{}", last_hash)])
                .current_dir(working_dir)
                .output();
            
            if let Ok(output) = output {
                if output.status.success() {
                    let count_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if let Ok(count) = count_str.parse::<usize>() {
                        if count >= 15 {
                            println!("{}", i18n.get("index_suggest_title").replace("{}", &count.to_string()).yellow());
                            println!("{}", i18n.get("index_suggest_action").yellow());
                            println!();
                        }
                    }
                }
            }
        } else {
            // No last hash, maybe never indexed or first run.
            // Check if it's a git repo
            if working_dir.join(".git").exists() {
                 // Suggest indexing for the first time
                 println!("{}", i18n.get("index_tip_title").blue());
                 println!();
            }
        }
    }
}

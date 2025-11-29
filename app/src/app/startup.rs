use super::review;
use anyhow::Result;
use api::ApiClient;
use config::Config;
use history::ChatSession;
use i18n::I18n;
use prompts;
use std::env;
use ui;

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

    if smart_approve {
        ui::set_smart_approval_mode(true);
    }

    // Check for --ally or --yolo flag (disabled if smart approval is active)
    let auto_approve = !smart_approve && env::args().any(|arg| arg == "--ally" || arg == "--yolo");

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

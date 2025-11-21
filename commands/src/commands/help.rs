use crate::i18n::I18n;
use colored::Colorize;

/// Print help information
pub fn print_help(i18n: &I18n) {
    println!("\n{}", i18n.get("help_title").bright_cyan().bold());
    println!("{}", "═".repeat(60).bright_black());

    // Model commands
    println!("\n{}", i18n.get("help_model").yellow().bold());
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/model list".cyan(),
        i18n.get("cmd_model_list").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/model switch <name>".cyan(),
        i18n.get("cmd_model_switch").dimmed()
    );

    // History commands
    println!("\n{}", i18n.get("help_history").yellow().bold());
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/history list".cyan(),
        i18n.get("cmd_history_list").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/history new".cyan(),
        i18n.get("cmd_history_new").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/history switch <id>".cyan(),
        i18n.get("cmd_history_switch").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/history del <id>".cyan(),
        i18n.get("cmd_history_del").dimmed()
    );

    // Language commands
    println!("\n{}", i18n.get("help_language").yellow().bold());
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/language ui <lang>".cyan(),
        i18n.get("cmd_language_ui").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/language ai <lang>".cyan(),
        i18n.get("cmd_language_ai").dimmed()
    );

    // Other commands
    println!("\n{}", i18n.get("help_other").yellow().bold());
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/help".cyan(),
        i18n.get("cmd_help").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/exit".cyan(),
        i18n.get("cmd_exit").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/agents.md".cyan(),
        i18n.get("cmd_agents_md").dimmed()
    );

    // Run command
    println!("\n{}", i18n.get("help_runcommand").yellow().bold());
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/runcommand list".cyan(),
        i18n.get("cmd_runcommand_list").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/runcommand add <cmd>".cyan(),
        i18n.get("cmd_runcommand_add").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/runcommand del <cmd>".cyan(),
        i18n.get("cmd_runcommand_del").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/runcommand info <id>".cyan(),
        i18n.get("cmd_runcommand_info").dimmed()
    );

    println!("\n{}", "═".repeat(60).bright_black());
    println!();
}

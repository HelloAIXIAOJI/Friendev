use std::io::{self, Write};

use crate::ui::get_i18n;

/// Handle content output
pub fn print_content(text: &str, has_reasoning: &mut bool) -> std::io::Result<()> {
    // If there was reasoning before, reset color and newline
    if *has_reasoning {
        print!("\x1b[0m\n\n"); // Reset color and newline
        *has_reasoning = false;
    }
    print!("{}", text);
    io::stdout().flush()
}

/// Handle reasoning output
pub fn print_reasoning(
    text: &str,
    is_first_reasoning: &mut bool,
    has_reasoning: &mut bool,
) -> std::io::Result<()> {
    let i18n = get_i18n();
    if *is_first_reasoning {
        print!("\x1b[90m[{}] ", i18n.get("chat_think_label")); // Dark gray hint
        *is_first_reasoning = false;
    }
    print!("\x1b[90m{}", text); // Dark gray for reasoning
    io::stdout().flush()?;
    *has_reasoning = true;
    Ok(())
}

/// Print AI prefix
pub fn print_ai_prefix() -> std::io::Result<()> {
    let i18n = get_i18n();
    print!("\n\x1b[36m[{}]\x1b[0m ", i18n.get("chat_ai_label"));
    io::stdout().flush()
}

/// Print tool call separator
pub fn print_tool_call_separator() -> std::io::Result<()> {
    println!();
    Ok(())
}

/// Finalize output formatting
pub fn finalize_output(has_reasoning: bool, content_empty: bool) -> std::io::Result<()> {
    // Ensure color is reset at the end and newline
    if has_reasoning {
        print!("\x1b[0m\n");
    } else if !content_empty {
        // If there's normal output, newline
        println!();
    }
    Ok(())
}

/// Print tool parsing error
pub fn print_tool_parse_error() {
    let i18n = get_i18n();
    eprintln!(
        "\n\x1b[31m[✗] {}:\x1b[0m {}",
        i18n.get("error"),
        i18n.get("chat_tool_parse_error")
    );
    eprintln!(
        "\x1b[33m[!] {}:\x1b[0m {}\n",
        i18n.get("chat_debug_info_label"),
        i18n.get("chat_tool_parse_debug")
    );
}

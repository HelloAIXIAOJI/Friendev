use std::io;
use ui::{enhanced_output, get_i18n};

/// Handle content output
pub fn print_content(text: &str) -> std::io::Result<()> {
    enhanced_output::print_content(text)
}

/// Print AI prefix
pub fn print_ai_prefix() -> std::io::Result<()> {
    enhanced_output::print_ai_prefix()
}

/// Print tool call separator
pub fn print_tool_call_separator() -> std::io::Result<()> {
    println!();
    Ok(())
}

/// Finalize output formatting
pub fn finalize_output(content_empty: bool) -> std::io::Result<()> {
    // Ensure proper newline at the end
    if !content_empty {
        println!();
    }
    Ok(())
}

/// Print tool parsing error
pub fn print_tool_parse_error() {
    let i18n = get_i18n();
    let error_msg = format!(
        "{}: {}",
        i18n.get("error"),
        i18n.get("chat_tool_parse_error")
    );
    let _ = enhanced_output::print_error(&error_msg);
    
    let debug_msg = format!(
        "{}: {}",
        i18n.get("chat_debug_info_label"),
        i18n.get("chat_tool_parse_debug")
    );
    let _ = enhanced_output::print_warning(&debug_msg);
}

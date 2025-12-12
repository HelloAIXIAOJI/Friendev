use std::io::{self, Write};
use ui::{enhanced_output, get_i18n};

/// Reasoning buffer threshold - only print THINK if we have at least this many chars
const REASONING_BUFFER_THRESHOLD: usize = 5;

/// Handle content output
pub fn print_content(text: &str, has_reasoning: &mut bool) -> std::io::Result<()> {
    // If there was reasoning before, reset and add spacing
    if *has_reasoning {
        print!("\n\n");
        io::stdout().flush()?;
        *has_reasoning = false;
    }
    enhanced_output::print_content(text)
}

/// Handle reasoning output with buffering
/// Only prints THINK prefix when buffer reaches threshold
pub fn print_reasoning(
    text: &str,
    is_first_reasoning: &mut bool,
    has_reasoning: &mut bool,
    reasoning_buffer: &mut String,
) -> std::io::Result<()> {
    // If we haven't printed THINK yet, buffer the content
    if *is_first_reasoning {
        reasoning_buffer.push_str(text);
        
        // Check if buffer reached threshold
        if reasoning_buffer.chars().count() >= REASONING_BUFFER_THRESHOLD {
            // Now print THINK prefix and buffered content
            enhanced_output::print_reasoning_prefix()?;
            enhanced_output::print_reasoning_text(reasoning_buffer)?;
            reasoning_buffer.clear();
            *is_first_reasoning = false;
            *has_reasoning = true;
        }
    } else {
        // Already printed THINK, just stream normally
        enhanced_output::print_reasoning_text(text)?;
        *has_reasoning = true;
    }
    Ok(())
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
pub fn finalize_output(has_reasoning: bool, content_empty: bool) -> std::io::Result<()> {
    // Ensure proper newline at the end
    if has_reasoning {
        println!();
    } else if !content_empty {
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

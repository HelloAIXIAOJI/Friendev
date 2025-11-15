use std::io::{self, Write};

/// Handle content output
pub fn print_content(text: &str, has_reasoning: &mut bool) -> std::io::Result<()> {
    // If there was reasoning before, reset color and newline
    if *has_reasoning {
        print!("\x1b[0m\n\n");  // Reset color and newline
        *has_reasoning = false;
    }
    print!("{}", text);
    io::stdout().flush()
}

/// Handle reasoning output
pub fn print_reasoning(text: &str, is_first_reasoning: &mut bool, has_reasoning: &mut bool) -> std::io::Result<()> {
    if *is_first_reasoning {
        print!("\x1b[90m[THINK] ");  // Dark gray hint
        *is_first_reasoning = false;
    }
    print!("\x1b[90m{}", text);  // Dark gray for reasoning
    io::stdout().flush()?;
    *has_reasoning = true;
    Ok(())
}

/// Print AI prefix
pub fn print_ai_prefix() -> std::io::Result<()> {
    print!("\n\x1b[36m[AI]\x1b[0m ");
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
    eprintln!("\n\x1b[31m[✗] Error:\x1b[0m Tool calls detected but all failed to parse");
    eprintln!("\x1b[33m[!] Debug Info:\x1b[0m Check if tool arguments are valid JSON\n");
}

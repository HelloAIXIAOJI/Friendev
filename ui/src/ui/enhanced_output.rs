use crossterm::{
    cursor,
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

use super::get_i18n;

/// Box drawing characters
pub mod box_chars {
    pub const TOP_LEFT: &str = "╭";
    pub const TOP_RIGHT: &str = "╮";
    pub const BOTTOM_LEFT: &str = "╰";
    pub const BOTTOM_RIGHT: &str = "╯";
    pub const HORIZONTAL: &str = "─";
    pub const VERTICAL: &str = "│";
    pub const T_DOWN: &str = "┬";
    pub const T_UP: &str = "┴";
}

/// Spinner frames for animation
const SPINNER_FRAMES: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

/// Print a styled separator line
pub fn print_separator(width: Option<u16>) -> io::Result<()> {
    let term_width = width.unwrap_or_else(|| {
        terminal::size().map(|(w, _)| w).unwrap_or(80)
    });
    
    let line = box_chars::HORIZONTAL.repeat(term_width as usize);
    execute!(
        io::stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print(&line),
        ResetColor,
        Print("\n")
    )
}

/// Print a boxed section header
pub fn print_section_header(title: &str) -> io::Result<()> {
    let term_width = terminal::size().map(|(w, _)| w).unwrap_or(80) as usize;
    let title_width = title.width();
    let padding = 2; // spaces around title
    let total_content = title_width + padding * 2;
    
    if total_content >= term_width {
        // Fallback for narrow terminals
        execute!(
            io::stdout(),
            SetForegroundColor(Color::Cyan),
            Print(format!("─ {} ", title)),
            ResetColor,
            Print("\n")
        )?;
        return Ok(());
    }
    
    let remaining = term_width.saturating_sub(total_content + 2); // 2 for corners
    let left_line = box_chars::HORIZONTAL.repeat(1);
    let right_line = box_chars::HORIZONTAL.repeat(remaining.saturating_sub(1));
    
    execute!(
        io::stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print(box_chars::TOP_LEFT),
        Print(&left_line),
        Print(" "),
        SetForegroundColor(Color::Cyan),
        Print(title),
        SetForegroundColor(Color::DarkGrey),
        Print(" "),
        Print(&right_line),
        Print(box_chars::TOP_RIGHT),
        ResetColor,
        Print("\n")
    )
}

/// Print a boxed section footer
pub fn print_section_footer() -> io::Result<()> {
    let term_width = terminal::size().map(|(w, _)| w).unwrap_or(80) as usize;
    let line = box_chars::HORIZONTAL.repeat(term_width.saturating_sub(2));
    
    execute!(
        io::stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print(box_chars::BOTTOM_LEFT),
        Print(&line),
        Print(box_chars::BOTTOM_RIGHT),
        ResetColor,
        Print("\n")
    )
}

/// Print AI message prefix with enhanced styling
pub fn print_ai_prefix() -> io::Result<()> {
    let i18n = get_i18n();
    execute!(
        io::stdout(),
        Print("\n"),
        SetForegroundColor(Color::Cyan),
        Print("▍"),
        Print(format!(" {} ", i18n.get("chat_ai_label"))),
        ResetColor
    )
}

/// Print reasoning block with dim styling
pub fn print_reasoning_prefix() -> io::Result<()> {
    let i18n = get_i18n();
    execute!(
        io::stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print(format!("\n  {} ", i18n.get("chat_think_label"))),
        ResetColor
    )
}

/// Print reasoning text (dim gray)
pub fn print_reasoning_text(text: &str) -> io::Result<()> {
    execute!(
        io::stdout(),
        SetForegroundColor(Color::DarkGrey),
        Print(text),
        ResetColor
    )
}

/// Print normal content
pub fn print_content(text: &str) -> io::Result<()> {
    execute!(io::stdout(), Print(text))
}

/// Print error message with icon
pub fn print_error(message: &str) -> io::Result<()> {
    let i18n = get_i18n();
    execute!(
        io::stdout(),
        Print("\n"),
        SetForegroundColor(Color::Red),
        Print("✗ "),
        Print(i18n.get("error")),
        ResetColor,
        Print(": "),
        Print(message),
        Print("\n")
    )
}

/// Print warning message with icon
pub fn print_warning(message: &str) -> io::Result<()> {
    execute!(
        io::stdout(),
        Print("\n"),
        SetForegroundColor(Color::Yellow),
        Print("⚠ "),
        ResetColor,
        Print(message),
        Print("\n")
    )
}

/// Print success message with icon
pub fn print_success(message: &str) -> io::Result<()> {
    execute!(
        io::stdout(),
        Print("\n"),
        SetForegroundColor(Color::Green),
        Print("✓ "),
        ResetColor,
        Print(message),
        Print("\n")
    )
}

/// Tool call progress display
pub struct ToolProgress {
    name: String,
    argument: Option<String>,
    spinner_index: usize,
    line_start_col: u16,
}

impl ToolProgress {
    pub fn new(name: String, argument: Option<String>) -> Self {
        Self {
            name,
            argument,
            spinner_index: 0,
            line_start_col: 0,
        }
    }

    /// Start displaying the progress (saves cursor position)
    pub fn start(&mut self) -> io::Result<()> {
        let i18n = get_i18n();
        
        // Save starting position
        let (col, _) = cursor::position()?;
        self.line_start_col = col;
        
        execute!(
            io::stdout(),
            Print("\n  "),
            SetForegroundColor(Color::DarkGrey),
            Print(SPINNER_FRAMES[0]),
            Print(" "),
            ResetColor,
            SetForegroundColor(Color::DarkGrey),
            Print(i18n.get("tool_action_using")),
            Print(" "),
            ResetColor,
            SetForegroundColor(Color::Cyan),
            Print(&self.name),
            ResetColor
        )?;
        
        if let Some(arg) = &self.argument {
            execute!(
                io::stdout(),
                Print(" "),
                SetForegroundColor(Color::DarkGrey),
                Print(format!("({})", arg)),
                ResetColor
            )?;
        }
        
        io::stdout().flush()
    }

    /// Update the spinner animation
    pub fn tick(&mut self) -> io::Result<()> {
        self.spinner_index = (self.spinner_index + 1) % SPINNER_FRAMES.len();
        
        // Move to the start of the line and redraw
        execute!(
            io::stdout(),
            cursor::MoveToColumn(0),
            terminal::Clear(ClearType::CurrentLine),
            Print("  "),
            SetForegroundColor(Color::DarkGrey),
            Print(SPINNER_FRAMES[self.spinner_index]),
            Print(" "),
            ResetColor,
            SetForegroundColor(Color::DarkGrey),
            Print(super::get_i18n().get("tool_action_using")),
            Print(" "),
            ResetColor,
            SetForegroundColor(Color::Cyan),
            Print(&self.name),
            ResetColor
        )?;
        
        if let Some(arg) = &self.argument {
            execute!(
                io::stdout(),
                Print(" "),
                SetForegroundColor(Color::DarkGrey),
                Print(format!("({})", arg)),
                ResetColor
            )?;
        }
        
        io::stdout().flush()
    }

    /// Finish with success
    pub fn finish_success(&self, result: Option<&str>) -> io::Result<()> {
        let i18n = get_i18n();
        
        execute!(
            io::stdout(),
            cursor::MoveToColumn(0),
            terminal::Clear(ClearType::CurrentLine),
            Print("  "),
            SetForegroundColor(Color::Green),
            Print("✓"),
            Print(" "),
            ResetColor,
            SetForegroundColor(Color::DarkGrey),
            Print(i18n.get("tool_action_used")),
            Print(" "),
            ResetColor,
            SetForegroundColor(Color::Cyan),
            Print(&self.name),
            ResetColor
        )?;
        
        if let Some(arg) = &self.argument {
            execute!(
                io::stdout(),
                Print(" "),
                SetForegroundColor(Color::DarkGrey),
                Print(format!("({})", arg)),
                ResetColor
            )?;
        }
        
        execute!(io::stdout(), Print("\n"))?;
        
        if let Some(res) = result {
            execute!(
                io::stdout(),
                Print("    "),
                SetForegroundColor(Color::DarkGrey),
                Print(res),
                ResetColor,
                Print("\n")
            )?;
        }
        
        io::stdout().flush()
    }

    /// Finish with error
    pub fn finish_error(&self, error: Option<&str>) -> io::Result<()> {
        let i18n = get_i18n();
        
        execute!(
            io::stdout(),
            cursor::MoveToColumn(0),
            terminal::Clear(ClearType::CurrentLine),
            Print("  "),
            SetForegroundColor(Color::Red),
            Print("✗"),
            Print(" "),
            ResetColor,
            SetForegroundColor(Color::DarkGrey),
            Print(i18n.get("tool_action_used")),
            Print(" "),
            ResetColor,
            SetForegroundColor(Color::Cyan),
            Print(&self.name),
            ResetColor
        )?;
        
        if let Some(arg) = &self.argument {
            execute!(
                io::stdout(),
                Print(" "),
                SetForegroundColor(Color::DarkGrey),
                Print(format!("({})", arg)),
                ResetColor
            )?;
        }
        
        execute!(io::stdout(), Print("\n"))?;
        
        if let Some(err) = error {
            execute!(
                io::stdout(),
                Print("    "),
                SetForegroundColor(Color::Red),
                Print(err),
                ResetColor,
                Print("\n")
            )?;
        }
        
        io::stdout().flush()
    }
}

/// Print a list of tool calls in a boxed format
pub fn print_tool_calls_box(tools: &[(String, Option<String>)]) -> io::Result<()> {
    if tools.is_empty() {
        return Ok(());
    }
    
    let i18n = get_i18n();
    print_section_header(&i18n.get("tools_header"))?;
    
    for (name, arg) in tools {
        execute!(
            io::stdout(),
            SetForegroundColor(Color::DarkGrey),
            Print(box_chars::VERTICAL),
            Print(" • "),
            ResetColor,
            SetForegroundColor(Color::Cyan),
            Print(name),
            ResetColor
        )?;
        
        if let Some(a) = arg {
            execute!(
                io::stdout(),
                Print("  "),
                SetForegroundColor(Color::DarkGrey),
                Print(a),
                ResetColor
            )?;
        }
        
        execute!(io::stdout(), Print("\n"))?;
    }
    
    print_section_footer()
}

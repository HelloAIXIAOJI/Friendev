use colored::Colorize;
use std::io::{self, Write};

/// Spinner 动画状态
pub struct Spinner {
    frames: Vec<&'static str>,
    current: usize,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            current: 0,
        }
    }

    pub fn next_frame(&mut self) -> &'static str {
        let frame = self.frames[self.current];
        self.current = (self.current + 1) % self.frames.len();
        frame
    }

    pub fn render(&mut self, text: &str) {
        print!("\r  {} {}", self.next_frame().bright_black(), text.dimmed());
        io::stdout().flush().ok();
    }
}

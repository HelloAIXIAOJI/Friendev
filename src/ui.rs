use colored::Colorize;
use std::io::{self, Write};

/// UI 组件，用于展示工具调用的流式进度
#[derive(Clone)]
pub struct ToolCallDisplay {
    pub name: String,
    pub key_argument: Option<String>,
    pub is_finished: bool,
    pub is_success: bool,
    pub result_brief: Option<String>,
}

impl ToolCallDisplay {
    pub fn new(name: String) -> Self {
        Self {
            name,
            key_argument: None,
            is_finished: false,
            is_success: false,
            result_brief: None,
        }
    }

    /// 更新关键参数（如文件路径）
    pub fn update_argument(&mut self, arg: String) {
        self.key_argument = Some(arg);
    }

    /// 完成工具调用，设置结果
    pub fn finish(&mut self, success: bool, brief: Option<String>) {
        self.is_finished = true;
        self.is_success = success;
        self.result_brief = brief;
    }

    /// 渲染正在进行的状态（流式显示）
    pub fn render_streaming(&self) {
        let status = if self.is_finished {
            if self.is_success {
                "•".green()
            } else {
                "•".red()
            }
        } else {
            "⠋".bright_black()
        };

        let action = if self.is_finished { "Used" } else { "Using" };
        
        print!("\r  {} {} {}", 
            status, 
            action.dimmed(),
            self.name.cyan().bold()
        );

        if let Some(arg) = &self.key_argument {
            print!(" {}", format!("({})", arg).bright_black());
        }

        io::stdout().flush().ok();
    }

    /// 渲染最终状态
    pub fn render_final(&self) {
        let bullet = if self.is_success {
            "•".green()
        } else {
            "•".red()
        };

        println!("  {} {} {}", 
            bullet,
            "Used".dimmed(),
            self.name.cyan().bold()
        );

        if let Some(brief) = &self.result_brief {
            let style = if self.is_success {
                brief.bright_black()
            } else {
                brief.red()
            };
            println!("    {}", style);
        }
    }
}

/// 提取工具调用的关键参数
pub fn extract_key_argument(tool_name: &str, arguments: &str) -> Option<String> {
    // 尝试解析 JSON
    let json: serde_json::Value = match serde_json::from_str(arguments) {
        Ok(v) => v,
        Err(_) => return None,
    };

    let key = match tool_name {
        "file_read" | "file_write" => {
            json.get("path")
                .and_then(|v| v.as_str())
                .map(|s| normalize_path(s))
        }
        "file_list" => {
            json.get("path")
                .and_then(|v| v.as_str())
                .map(|s| normalize_path(s))
                .or_else(|| Some("./".to_string()))
        }
        _ => None,
    };

    key.map(|s| shorten_middle(&s, 50))
}

/// 规范化路径显示（简化为相对路径）
fn normalize_path(path: &str) -> String {
    use std::path::Path;
    
    let p = Path::new(path);
    
    // 如果是当前目录下的文件，使用相对路径
    if let Ok(cwd) = std::env::current_dir() {
        if let Ok(relative) = p.strip_prefix(&cwd) {
            return relative.display().to_string();
        }
    }
    
    path.to_string()
}

/// 缩短字符串中间部分
fn shorten_middle(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        return s.to_string();
    }
    
    let half = (max_len - 3) / 2;
    format!("{}...{}", &s[..half], &s[s.len() - half..])
}

/// 用户审批提示
/// 返回 (approved, always)
pub fn prompt_approval(action: &str, file_path: &str, content_preview: Option<&str>) -> io::Result<(bool, bool)> {
    use std::path::Path;
    
    // 提取文件名
    let file_name = Path::new(file_path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(file_path);
    
    println!();
    println!("{}", "  ──── Approval Required ──────────────────".yellow());
    println!("{}", format!("    {} wants to modify:", action).yellow());
    println!("{}", format!("      {}", file_name).yellow().bold());
    
    // 显示内容预览
    if let Some(preview) = content_preview {
        println!("{}", "                                           ".yellow());
        println!("{}", "    Content preview:".yellow());
        
        let lines: Vec<&str> = preview.lines().take(5).collect();
        for line in lines {
            let truncated = if line.len() > 35 {
                format!("{}...", &line[..35])
            } else {
                line.to_string()
            };
            println!("{}", format!("      {}", truncated).bright_black());
        }
        
        let total_lines = preview.lines().count();
        if total_lines > 5 {
            println!("{}", format!("      ... ({} more lines)", total_lines - 5).bright_black());
        }
    }
    
    println!("{}", "                                           ".yellow());
    println!("{}", "    [Y]es / [N]o / [A]lways                ".yellow());
    println!("{}", "  ─────────────────────────────────────────".yellow());
    print!("  {} ", "Your choice:".bright_cyan());
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    let choice = input.trim().to_lowercase();
    match choice.as_str() {
        "y" | "yes" => {
            Ok((true, false))
        }
        "a" | "always" => {
            println!("  {} Approved for this session", "✓".green());
            Ok((true, true))  // 返回 true, true 表示 always
        }
        _ => {
            println!("  {} Rejected", "✗".red());
            Ok((false, false))
        }
    }
}

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
        print!("\r  {} {}", 
            self.next_frame().bright_black(),
            text.dimmed()
        );
        io::stdout().flush().ok();
    }
}

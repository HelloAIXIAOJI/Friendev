use colored::Colorize;
use std::io::{self, Write};

use crate::ui::get_i18n;

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
        let i18n = get_i18n();

        let status = if self.is_finished {
            if self.is_success {
                "•".green()
            } else {
                "•".red()
            }
        } else {
            "⠋".bright_black()
        };

        let action = if self.is_finished {
            i18n.get("tool_action_used")
        } else {
            i18n.get("tool_action_using")
        };

        print!(
            "\r  {} {} {}",
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
        let i18n = get_i18n();

        let bullet = if self.is_success {
            "•".green()
        } else {
            "•".red()
        };

        println!(
            "  {} {} {}",
            bullet,
            i18n.get("tool_action_used").dimmed(),
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
        "file_read" | "file_write" => json
            .get("path")
            .and_then(|v| v.as_str())
            .map(|s| normalize_path(s)),
        "file_list" => json
            .get("path")
            .and_then(|v| v.as_str())
            .map(|s| normalize_path(s))
            .or_else(|| Some("./".to_string())),
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
    let char_count = s.chars().count();
    if char_count <= max_len {
        return s.to_string();
    }

    let half = (max_len - 3) / 2;
    let start: String = s.chars().take(half).collect();
    let end: String = s.chars().skip(char_count - half).collect();
    format!("{}...{}", start, end)
}

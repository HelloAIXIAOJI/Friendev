use std::io;

use super::enhanced_output::ToolProgress;

/// UI 组件，用于展示工具调用的流式进度
#[derive(Clone)]
pub struct ToolCallDisplay {
    pub name: String,
    pub key_argument: Option<String>,
    pub is_finished: bool,
    pub is_success: bool,
    pub result_brief: Option<String>,
    /// 用于防止重复渲染的标志
    pub has_rendered: bool,
    /// 上次渲染的参数，用于检测变化
    pub last_rendered_arg: Option<String>,
}

impl ToolCallDisplay {
    pub fn new(name: String) -> Self {
        Self {
            name,
            key_argument: None,
            is_finished: false,
            is_success: false,
            result_brief: None,
            has_rendered: false,
            last_rendered_arg: None,
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

    /// 渲染正在进行的状态（流式显示）- 使用 ToolProgress 实现
    /// 返回 true 表示进行了渲染，false 表示跳过了渲染
    pub fn render_streaming(&mut self) -> bool {
        // 已完成的不再显示
        if self.is_finished {
            return false;
        }
        
        // 检查是否需要渲染：
        // 1. 如果从未渲染过，需要渲染
        // 2. 如果参数发生了变化，需要更新渲染
        let should_render = !self.has_rendered || 
                           (self.key_argument != self.last_rendered_arg);
        
        if !should_render {
            return false;
        }
        
        // 创建 ToolProgress 并启动
        let mut progress = ToolProgress::new(self.name.clone(), self.key_argument.clone());
        let _ = progress.start();
        
        // 更新渲染状态
        self.has_rendered = true;
        self.last_rendered_arg = self.key_argument.clone();
        
        true
    }

    /// 渲染最终状态
    pub fn render_final(&self) {
        let progress = ToolProgress::new(self.name.clone(), self.key_argument.clone());
        
        let result: io::Result<()> = if self.is_success {
            progress.finish_success(self.result_brief.as_deref())
        } else {
            progress.finish_error(self.result_brief.as_deref())
        };
        
        let _ = result;
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
            .map(normalize_path),
        "file_list" => json
            .get("path")
            .and_then(|v| v.as_str())
            .map(normalize_path)
            .or_else(|| Some("./".to_string())),
        "network_search_auto" | "network_search_duckduckgo" | "network_search_bing" => json
            .get("keywords")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        _ => None,
    };

    key.map(|s| shorten_middle(s.as_str(), 50))
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

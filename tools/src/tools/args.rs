use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoItem {
    pub id: String,
    pub content: String,
    pub status: String, // "pending" | "in_progress" | "completed"
    #[serde(default = "default_priority")]
    pub priority: String, // "high" | "medium" | "low"
}

fn default_priority() -> String {
    "medium".to_string()
}

#[derive(Debug, Deserialize)]
pub struct TodoWriteArgs {
    pub todos: Vec<TodoItem>,
}


#[derive(Debug, Deserialize)]
pub struct FileListArgs {
    pub path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FileReadArgs {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct FileSearchArgs {
    pub pattern: String,
    pub path: Option<String>,
    pub include: Option<String>,
    #[serde(default)]
    pub ignore_case: bool,
}

#[derive(Debug, Deserialize)]
pub struct FileOutlineArgs {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct FileSearchByOutlineArgs {
    pub pattern: String,
}

#[derive(Debug, Deserialize)]
pub struct IndexFileArgs {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct FileWriteArgs {
    pub path: String,
    pub content: String,
    #[serde(default = "default_write_mode")]
    pub mode: String, // "overwrite" 或 "append"
}

pub fn default_write_mode() -> String {
    "overwrite".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Edit {
    pub old: String,
    pub new: String,
    #[serde(default)]
    pub replace_all: bool,
    #[serde(default)]
    pub normalize: bool, // 是否启用宽松匹配（忽略多余空格/换行符差异）
    #[serde(default)]
    pub regex: bool, // 是否使用正则表达式匹配
}

#[derive(Debug, Deserialize)]
pub struct FileReplaceArgs {
    pub path: String,
    pub edits: Vec<Edit>,
}

#[derive(Debug, Deserialize)]
pub struct SearchArgs {
    pub keywords: String,
    #[serde(default = "default_max_results")]
    pub max_results: usize,
}

pub fn default_max_results() -> usize {
    5
}

#[derive(Debug, Deserialize, Clone)]
pub struct DiffHunk {
    pub start_line: usize,   // 开始行号（从1开始）
    pub num_lines: usize,    // 原文件中的行数
    pub new_content: String, // 新内容（完整文本）
}

#[derive(Debug, Deserialize)]
pub struct FileDiffEditArgs {
    pub path: String,
    pub hunks: Vec<DiffHunk>, // 多个 hunk 编辑
}

#[derive(Debug, Deserialize)]
pub struct RunCommandArgs {
    pub command: String,
    #[serde(default)]
    pub background: bool, // 是否后台运行
}

#[derive(Debug, Deserialize)]
pub struct FetchUrlArgs {
    pub url: String,
    #[serde(default)]
    pub max_bytes: Option<usize>,
}

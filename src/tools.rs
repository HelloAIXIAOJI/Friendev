use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::collections::HashSet;

/// 会话级审批状态
static APPROVED_ACTIONS: Mutex<Option<HashSet<String>>> = Mutex::new(None);

/// 检查操作是否已被批准
fn is_action_approved(action: &str) -> bool {
    let mut approved = APPROVED_ACTIONS.lock().unwrap();
    if approved.is_none() {
        *approved = Some(HashSet::new());
    }
    approved.as_ref().unwrap().contains(action)
}

/// 添加操作到已批准列表
fn approve_action_for_session(action: &str) {
    let mut approved = APPROVED_ACTIONS.lock().unwrap();
    if approved.is_none() {
        *approved = Some(HashSet::new());
    }
    approved.as_mut().unwrap().insert(action.to_string());
}

/// 工具执行结果
pub struct ToolResult {
    pub success: bool,
    pub brief: String,
    pub output: String,
}

impl ToolResult {
    pub fn ok(brief: String, output: String) -> Self {
        Self { success: true, brief, output }
    }

    pub fn error(brief: String) -> Self {
        Self { success: false, brief: brief.clone(), output: brief }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

pub fn get_available_tools() -> Vec<Tool> {
    vec![
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_list".to_string(),
                description: "List all files and subdirectories in the specified directory".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Directory path (optional, defaults to working directory)"
                        }
                    },
                    "required": []
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_read".to_string(),
                description: "Read the content of a file".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "File path to read"
                        }
                    },
                    "required": ["path"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_write".to_string(),
                description: "Write content to a file. When content is too long (>100 lines), use multiple calls: first with 'overwrite', then with 'append' mode.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "File path to write"
                        },
                        "content": {
                            "type": "string",
                            "description": "Content to write"
                        },
                        "mode": {
                            "type": "string",
                            "enum": ["overwrite", "append"],
                            "description": "Write mode: 'overwrite' to replace file content (default), 'append' to add to end of file",
                            "default": "overwrite"
                        }
                    },
                    "required": ["path", "content"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_replace".to_string(),
                description: "Replace strings in a file, supporting batch edits. Prefer this tool over file_write to modify existing files.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "File path to edit"
                        },
                        "edits": {
                            "type": "array",
                            "description": "List of edit operations to apply in order",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "old": {
                                        "type": "string",
                                        "description": "Old string to replace (supports multi-line)"
                                    },
                                    "new": {
                                        "type": "string",
                                        "description": "New string (supports multi-line)"
                                    },
                                    "replace_all": {
                                        "type": "boolean",
                                        "description": "Whether to replace all matches (default false, replaces only the first)",
                                        "default": false
                                    }
                                },
                                "required": ["old", "new"]
                            }
                        }
                    },
                    "required": ["path", "edits"]
                }),
            },
        },
    ]
}

#[derive(Debug, Deserialize)]
struct FileListArgs {
    path: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FileReadArgs {
    path: String,
}

#[derive(Debug, Deserialize)]
struct FileWriteArgs {
    path: String,
    content: String,
    #[serde(default = "default_write_mode")]
    mode: String,  // "overwrite" 或 "append"
}

fn default_write_mode() -> String {
    "overwrite".to_string()
}

#[derive(Debug, Deserialize)]
struct Edit {
    old: String,
    new: String,
    #[serde(default)]
    replace_all: bool,
}

#[derive(Debug, Deserialize)]
struct FileReplaceArgs {
    path: String,
    edits: Vec<Edit>,
}

pub fn execute_tool(name: &str, arguments: &str, working_dir: &Path, require_approval: bool) -> Result<ToolResult> {
    match name {
        "file_list" => {
            let args: FileListArgs = serde_json::from_str(arguments)
                .unwrap_or(FileListArgs { path: None });
            
            let target_path = if let Some(path) = args.path {
                let p = Path::new(&path);
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    working_dir.join(p)
                }
            } else {
                working_dir.to_path_buf()
            };

            if !target_path.exists() {
                return Ok(ToolResult::error(format!("路径不存在: {}", target_path.display())));
            }

            if !target_path.is_dir() {
                return Ok(ToolResult::error(format!("不是目录: {}", target_path.display())));
            }

            let mut items = Vec::new();
            for entry in fs::read_dir(&target_path)? {
                let entry = entry?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let item_type = if path.is_dir() { "目录" } else { "文件" };
                
                let metadata = entry.metadata()?;
                let size = if metadata.is_file() {
                    format_size(metadata.len())
                } else {
                    "-".to_string()
                };

                items.push(format!("{} [{}] ({})", name, item_type, size));
            }

            items.sort();

            let brief = if items.is_empty() {
                format!("目录为空")
            } else {
                format!("列出 {} 项", items.len())
            };

            let output = format!(
                "目录: {}\n共 {} 项:\n\n{}",
                target_path.display(),
                items.len(),
                items.join("\n")
            );

            Ok(ToolResult::ok(brief, output))
        }
        "file_read" => {
            let args: FileReadArgs = serde_json::from_str(arguments)?;
            
            let target_path = {
                let p = Path::new(&args.path);
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    working_dir.join(p)
                }
            };
            
            if !target_path.exists() {
                return Ok(ToolResult::error(format!("文件不存在: {}", target_path.display())));
            }
            
            if !target_path.is_file() {
                return Ok(ToolResult::error(format!("不是文件: {}", target_path.display())));
            }
            
            let content = fs::read_to_string(&target_path)?;
            let lines = content.lines().count();
            let bytes = content.len();
            
            let brief = format!("读取 {} 行, {} 字节", lines, bytes);
            let output = format!("文件: {}\n内容:\n{}", target_path.display(), content);
            
            Ok(ToolResult::ok(brief, output))
        }
        "file_write" => {
            let args: FileWriteArgs = serde_json::from_str(arguments)?;
            
            let target_path = {
                let p = Path::new(&args.path);
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    working_dir.join(p)
                }
            };
            
            // 验证 mode 参数
            let mode = args.mode.as_str();
            if mode != "overwrite" && mode != "append" {
                return Ok(ToolResult::error(format!("无效的写入模式: {}，只支持 'overwrite' 或 'append'", mode)));
            }
            
            // 危险操作：需要用户确认
            if require_approval && !is_action_approved("file_write") {
                use crate::ui::prompt_approval;
                let action_desc = if mode == "append" {
                    format!("追加到文件: {}", target_path.display())
                } else {
                    format!("覆盖文件: {}", target_path.display())
                };
                
                let (approved, always) = prompt_approval(
                    "WriteFile",
                    &action_desc,
                    Some(&args.content)  // 传递内容预览
                )?;
                
                if !approved {
                    return Ok(ToolResult::error("用户拒绝了该操作".to_string()));
                }
                
                // 如果用户选择 Always，保存状态
                if always {
                    approve_action_for_session("file_write");
                }
            }
            
            // 创建父目录（如果不存在）
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // 根据模式写入或追加
            if mode == "append" {
                // 追加模式
                use std::fs::OpenOptions;
                use std::io::Write;
                
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&target_path)?;
                file.write_all(args.content.as_bytes())?;
                
                let file_size = target_path.metadata()?.len();
                let brief = format!("追加 {} 字节", args.content.len());
                let output = format!(
                    "成功追加到文件: {}\n追加: {} 字节\n当前大小: {} 字节",
                    target_path.display(),
                    args.content.len(),
                    file_size
                );
                Ok(ToolResult::ok(brief, output))
            } else {
                // 覆盖模式
                fs::write(&target_path, &args.content)?;
                
                let brief = format!("写入 {} 字节", args.content.len());
                let output = format!(
                    "成功写入文件: {}\n大小: {} 字节",
                    target_path.display(),
                    args.content.len()
                );
                Ok(ToolResult::ok(brief, output))
            }
        }
        "file_replace" => {
            let args: FileReplaceArgs = serde_json::from_str(arguments)?;
            
            let target_path = {
                let p = Path::new(&args.path);
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    working_dir.join(p)
                }
            };
            
            // 验证文件存在
            if !target_path.exists() {
                return Ok(ToolResult::error(format!("文件不存在: {}", target_path.display())));
            }
            
            if !target_path.is_file() {
                return Ok(ToolResult::error(format!("不是文件: {}", target_path.display())));
            }
            
            // 需要审批（file_replace 也是危险操作）
            if require_approval && !is_action_approved("file_replace") {
                use crate::ui::prompt_approval;
                
                // 生成预览内容
                let preview = args.edits.iter()
                    .take(3)  // 最多显示 3 个编辑
                    .map(|e| {
                        let old_preview = if e.old.chars().count() > 40 {
                            let truncated: String = e.old.chars().take(40).collect();
                            format!("{}...", truncated)
                        } else {
                            e.old.clone()
                        };
                        let new_preview = if e.new.chars().count() > 40 {
                            let truncated: String = e.new.chars().take(40).collect();
                            format!("{}...", truncated)
                        } else {
                            e.new.clone()
                        };
                        format!("- Replace: {}\n  With: {}", old_preview, new_preview)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                
                let full_preview = if args.edits.len() > 3 {
                    format!("{}\n... and {} more edits", preview, args.edits.len() - 3)
                } else {
                    preview
                };
                
                let (approved, always) = prompt_approval(
                    "ReplaceFile",
                    &target_path.display().to_string(),
                    Some(&full_preview)
                )?;
                
                if !approved {
                    return Ok(ToolResult::error("用户拒绝了该操作".to_string()));
                }
                
                if always {
                    approve_action_for_session("file_replace");
                }
            }
            
            // 读取文件
            let mut content = fs::read_to_string(&target_path)?;
            let original_content = content.clone();
            
            // 应用所有编辑
            let mut replacements_made = 0;
            for edit in &args.edits {
                content = if edit.replace_all {
                    let count = content.matches(&edit.old).count();
                    replacements_made += count;
                    content.replace(&edit.old, &edit.new)
                } else {
                    if content.contains(&edit.old) {
                        replacements_made += 1;
                        content.replacen(&edit.old, &edit.new, 1)
                    } else {
                        content
                    }
                };
            }
            
            // 检查是否有修改
            if content == original_content {
                return Ok(ToolResult::error(
                    format!("未找到要替换的字符串。请确认 'old' 字符串与文件内容完全匹配。")
                ));
            }
            
            // 写回文件
            fs::write(&target_path, &content)?;
            
            let brief = format!("应用了 {} 个编辑，{} 个替换", args.edits.len(), replacements_made);
            let output = format!(
                "文件已更新: {}\n应用了 {} 个编辑\n共进行了 {} 个替换",
                target_path.display(),
                args.edits.len(),
                replacements_made
            );
            
            Ok(ToolResult::ok(brief, output))
        }
        _ => Ok(ToolResult::error(format!("未知工具: {}", name))),
    }
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// 自动生成工具列表描述，用于系统提示词
pub fn get_tools_description() -> String {
    let tools = get_available_tools();
    let mut descriptions = Vec::new();
    
    for tool in tools {
        descriptions.push(format!("- {}: {}", tool.function.name, tool.function.description));
    }
    
    descriptions.join("\n")
}

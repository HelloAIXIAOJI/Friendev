use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;

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
                description: "列出指定目录下的所有文件和子目录".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "目录路径（可选，默认为工作目录）"
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
                description: "读取文件的内容".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "要读取的文件路径"
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
                description: "写入内容到文件，如果文件不存在则创建".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "要写入的文件路径"
                        },
                        "content": {
                            "type": "string",
                            "description": "要写入的内容"
                        }
                    },
                    "required": ["path", "content"]
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
}

pub fn execute_tool(name: &str, arguments: &str, working_dir: &Path) -> Result<String> {
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
                return Ok(format!("错误: 路径不存在: {}", target_path.display()));
            }

            if !target_path.is_dir() {
                return Ok(format!("错误: 不是目录: {}", target_path.display()));
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

            let result = if items.is_empty() {
                format!("目录为空: {}", target_path.display())
            } else {
                format!(
                    "目录: {}\n共 {} 项:\n\n{}",
                    target_path.display(),
                    items.len(),
                    items.join("\n")
                )
            };

            Ok(result)
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
                return Ok(format!("错误: 文件不存在: {}", target_path.display()));
            }
            
            if !target_path.is_file() {
                return Ok(format!("错误: 不是文件: {}", target_path.display()));
            }
            
            let content = fs::read_to_string(&target_path)?;
            Ok(format!("文件: {}\n内容:\n{}", target_path.display(), content))
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
            
            // 创建父目录（如果不存在）
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            fs::write(&target_path, &args.content)?;
            
            Ok(format!(
                "成功写入文件: {}\n大小: {} 字节",
                target_path.display(),
                args.content.len()
            ))
        }
        _ => Ok(format!("未知工具: {}", name)),
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

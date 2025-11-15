use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::tools::types::ToolResult;
use crate::tools::args::FileReadArgs;
use super::file_common::normalize_path;

pub async fn execute_file_read(
    arguments: &str,
    working_dir: &Path,
) -> Result<ToolResult> {
    let args: FileReadArgs = serde_json::from_str(arguments)?;
    
    let target_path = normalize_path(&args.path, working_dir);
    
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

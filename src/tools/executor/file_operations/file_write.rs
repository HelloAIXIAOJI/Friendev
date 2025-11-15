use anyhow::Result;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use crate::tools::types::ToolResult;
use crate::tools::args::FileWriteArgs;
use super::file_common::{normalize_path, handle_approval_with_details};

pub async fn execute_file_write(
    arguments: &str,
    working_dir: &Path,
    require_approval: bool,
) -> Result<ToolResult> {
    let args: FileWriteArgs = serde_json::from_str(arguments)?;
    
    let target_path = normalize_path(&args.path, working_dir);
    
    // 验证 mode 参数
    let mode = args.mode.as_str();
    if mode != "overwrite" && mode != "append" {
        return Ok(ToolResult::error(format!(
            "无效的写入模式: {}，只支持 'overwrite' 或 'append'",
            mode
        )));
    }
    
    // 处理审批流程
    let action_desc = if mode == "append" {
        format!("追加到文件: {}", target_path.display())
    } else {
        format!("覆盖文件: {}", target_path.display())
    };
    
    if let Some(err) = handle_approval_with_details(
        "file_write",
        &action_desc,
        Some(&args.content),
        &target_path.display().to_string(),
        &args.content,
        require_approval,
    ).await? {
        return Ok(ToolResult::error(err));
    }
    
    // 创建父目录（如果不存在）
    if let Some(parent) = target_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // 根据模式写入或追加
    if mode == "append" {
        execute_append_mode(&target_path, &args.content)
    } else {
        execute_overwrite_mode(&target_path, &args.content)
    }
}

fn execute_append_mode(target_path: &Path, content: &str) -> Result<ToolResult> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(target_path)?;
    file.write_all(content.as_bytes())?;
    
    let file_size = target_path.metadata()?.len();
    let brief = format!("追加 {} 字节", content.len());
    let output = format!(
        "成功追加到文件: {}\n追加: {} 字节\n当前大小: {} 字节",
        target_path.display(),
        content.len(),
        file_size
    );
    Ok(ToolResult::ok(brief, output))
}

fn execute_overwrite_mode(target_path: &Path, content: &str) -> Result<ToolResult> {
    fs::write(target_path, content)?;
    
    let brief = format!("写入 {} 字节", content.len());
    let output = format!(
        "成功写入文件: {}\n大小: {} 字节",
        target_path.display(),
        content.len()
    );
    Ok(ToolResult::ok(brief, output))
}

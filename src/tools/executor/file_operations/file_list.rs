use anyhow::Result;
use std::fs;
use std::path::Path;

use crate::tools::types::ToolResult;
use crate::tools::args::FileListArgs;
use super::file_common::normalize_path;

pub async fn execute_file_list(
    arguments: &str,
    working_dir: &Path,
) -> Result<ToolResult> {
    let args: FileListArgs = serde_json::from_str(arguments)
        .unwrap_or(FileListArgs { path: None });
    
    let target_path = if let Some(path) = args.path {
        normalize_path(&path, working_dir)
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
            crate::tools::utils::format_size(metadata.len())
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

use anyhow::Result;
use std::fs;
use std::path::Path;

use super::file_common::normalize_path;
use crate::tools::args::FileListArgs;
use crate::tools::types::ToolResult;
use crate::ui::get_i18n;

pub async fn execute_file_list(arguments: &str, working_dir: &Path) -> Result<ToolResult> {
    let args: FileListArgs = serde_json::from_str(arguments).unwrap_or(FileListArgs { path: None });

    let i18n = get_i18n();

    let target_path = if let Some(path) = args.path {
        normalize_path(&path, working_dir)
    } else {
        working_dir.to_path_buf()
    };

    if !target_path.exists() {
        let tmpl = i18n.get("file_path_not_exist");
        return Ok(ToolResult::error(
            tmpl.replace("{}", &target_path.display().to_string()),
        ));
    }

    if !target_path.is_dir() {
        let tmpl = i18n.get("file_not_directory");
        return Ok(ToolResult::error(
            tmpl.replace("{}", &target_path.display().to_string()),
        ));
    }

    let mut items = Vec::new();
    for entry in fs::read_dir(&target_path)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let item_type = if path.is_dir() {
            i18n.get("file_item_type_dir")
        } else {
            i18n.get("file_item_type_file")
        };

        let metadata = entry.metadata()?;
        let size = if metadata.is_file() {
            crate::tools::utils::format_size(metadata.len())
        } else {
            i18n.get("file_item_size_na")
        };

        let tmpl = i18n.get("file_list_item");
        let line = tmpl
            .replacen("{}", &name, 1)
            .replacen("{}", &item_type, 1)
            .replacen("{}", &size, 1);
        items.push(line);
    }

    items.sort();

    let brief = if items.is_empty() {
        i18n.get("file_list_empty")
    } else {
        let tmpl = i18n.get("file_list_brief");
        tmpl.replace("{}", &items.len().to_string())
    };

    let header_tmpl = i18n.get("file_list_header");
    let header = header_tmpl.replace("{}", &target_path.display().to_string());
    let output = format!(
        "{}\n{}\n\n{}",
        header,
        i18n.get("file_list_count")
            .replace("{}", &items.len().to_string()),
        items.join("\n")
    );

    Ok(ToolResult::ok(brief, output))
}

use anyhow::Result;
use std::fs;
use std::path::Path;

use super::file_common::normalize_path;
use crate::tools::args::FileReadArgs;
use crate::tools::types::ToolResult;
use crate::ui::get_i18n;

pub async fn execute_file_read(arguments: &str, working_dir: &Path) -> Result<ToolResult> {
    let args: FileReadArgs = serde_json::from_str(arguments)?;

    let target_path = normalize_path(&args.path, working_dir);
    let i18n = get_i18n();

    if !target_path.exists() {
        let tmpl = i18n.get("file_not_exist");
        return Ok(ToolResult::error(
            tmpl.replace("{}", &target_path.display().to_string()),
        ));
    }

    if !target_path.is_file() {
        let tmpl = i18n.get("file_not_file");
        return Ok(ToolResult::error(
            tmpl.replace("{}", &target_path.display().to_string()),
        ));
    }

    let content = fs::read_to_string(&target_path)?;
    let lines = content.lines().count();
    let bytes = content.len();

    let brief_tmpl = i18n.get("file_read_brief");
    let brief =
        brief_tmpl
            .replacen("{}", &lines.to_string(), 1)
            .replacen("{}", &bytes.to_string(), 1);

    let header_tmpl = i18n.get("file_read_header");
    let header = header_tmpl.replace("{}", &target_path.display().to_string());
    let output = format!("{}\n{}", header, content);

    Ok(ToolResult::ok(brief, output))
}

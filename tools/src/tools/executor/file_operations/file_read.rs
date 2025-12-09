use anyhow::Result;
use std::fs;
use std::path::Path;

use super::file_common::normalize_path;
use crate::tools::args::FileReadArgs;
use crate::tools::types::ToolResult;
use ui::get_i18n;

pub async fn execute_file_read(arguments: &str, working_dir: &Path) -> Result<ToolResult> {
    execute_file_read_with_mcp(arguments, working_dir, None).await
}

pub async fn execute_file_read_with_mcp(
    arguments: &str, 
    working_dir: &Path, 
    mcp_integration: Option<&mcp::McpIntegration>
) -> Result<ToolResult> {
    let args: FileReadArgs = serde_json::from_str(arguments)?;
    let i18n = get_i18n();

    // Check if it's an MCP resource URI (format: mcp://server/resource)
    if args.path.starts_with("mcp://") {
        if let Some(integration) = mcp_integration {
            let uri = &args.path[6..]; // Remove "mcp://" prefix
            if let Some(pos) = uri.find('/') {
                let server = &uri[..pos];
                let resource = &uri[pos + 1..];
                
                match integration.read_resource(resource, Some(server)).await {
                    Ok(content) => {
                        let brief = format!("Read MCP resource from {}", server);
                        return Ok(ToolResult::ok(brief, content));
                    }
                    Err(e) => {
                        return Ok(ToolResult::error(format!("MCP resource error: {}", e)));
                    }
                }
            } else {
                return Ok(ToolResult::error("Invalid MCP URI format. Use: mcp://server/resource".to_string()));
            }
        } else {
            return Ok(ToolResult::error("MCP integration not available".to_string()));
        }
    }

    let target_path = normalize_path(&args.path, working_dir);

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
    let all_lines: Vec<&str> = content.lines().collect();
    let total_lines = all_lines.len();
    let total_bytes = content.len();

    let (output_content, read_lines_count) = if args.start_line.is_some() || args.end_line.is_some() {
        let start = args.start_line.unwrap_or(1);
        let end = args.end_line.unwrap_or(total_lines);

        if start < 1 || start > total_lines {
            return Ok(ToolResult::error(format!(
                "Start line {} is out of bounds (1-{}).",
                start, total_lines
            )));
        }
        
        if end < start {
             return Ok(ToolResult::error(format!(
                "End line {} is smaller than start line {}.",
                end, start
            )));
        }

        let start_idx = start - 1;
        let end_idx = std::cmp::min(end, total_lines);
        
        let selected_lines = &all_lines[start_idx..end_idx];
        let selected_text = selected_lines.join("\n");
        
        let header = format!("(Lines {}-{})", start, end_idx);
        (format!("{}\n{}", header, selected_text), selected_lines.len())
    } else {
        (content, total_lines)
    };

    let brief_tmpl = i18n.get("file_read_brief");
    let brief = brief_tmpl
        .replacen("{}", &read_lines_count.to_string(), 1)
        .replacen("{}", &total_bytes.to_string(), 1); // Note: bytes is still total file bytes for info

    let header_tmpl = i18n.get("file_read_header");
    let header = header_tmpl.replace("{}", &target_path.display().to_string());
    let output = format!("{}\n{}", header, output_content);

    Ok(ToolResult::ok(brief, output))
}

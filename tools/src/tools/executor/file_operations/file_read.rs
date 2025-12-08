use anyhow::Result;
use std::fs;
use std::path::Path;

use super::file_common::normalize_path;
use crate::tools::args::FileReadArgs;
use crate::types::ToolResult;
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

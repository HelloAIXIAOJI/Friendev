use anyhow::Result;
use std::path::Path;

use super::super::parser;
use super::file_common::normalize_path;
use crate::tools::args::FileOutlineArgs;
use crate::tools::types::ToolResult;

pub async fn execute_file_outline(arguments: &str, working_dir: &Path) -> Result<ToolResult> {
    let args: FileOutlineArgs = serde_json::from_str(arguments)?;
    let path = normalize_path(&args.path, working_dir);

    match parser::extract_symbols(&path) {
        Ok(symbols) => {
            if symbols.is_empty() {
                return Ok(ToolResult::ok(
                    format!("No symbols found in {}", path.display()),
                    "No symbols found.".to_string()
                ));
            }

            let mut results = String::new();
            results.push_str(&format!("Outline for {}:\n", path.display()));
            results.push_str("---\n");

            for symbol in &symbols {
                results.push_str(&format!("L{:<4} [{}] {}\n", symbol.line, symbol.kind, symbol.name));
            }

            let brief = format!("Found {} symbols in {}", symbols.len(), path.display());
            Ok(ToolResult::ok(brief, results))
        }
        Err(e) => Ok(ToolResult::error(format!("Failed to extract symbols: {}", e))),
    }
}

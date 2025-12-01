use anyhow::Result;
use std::path::Path;

use crate::tools::args::{FileSearchByOutlineArgs, IndexFileArgs};
use crate::tools::indexer::Indexer;
use crate::tools::executor::file_operations::file_common::normalize_path;
use crate::types::ToolResult;

pub async fn execute_file_search_by_outline(arguments: &str, working_dir: &Path) -> Result<ToolResult> {
    let args: FileSearchByOutlineArgs = serde_json::from_str(arguments)?;
    let indexer = Indexer::new(working_dir)?;
    
    match indexer.search_symbols(&args.pattern) {
        Ok(results) => {
            if results.is_empty() {
                return Ok(ToolResult::ok(
                    format!("No symbols found matching '{}' in outline index.", args.pattern),
                    "No symbols found. Try running '/index outline' if you suspect the index is outdated.".to_string()
                ));
            }

            let mut output = String::new();
            output.push_str(&format!("Found {} symbols matching '{}':\n", results.len(), args.pattern));
            output.push_str("---\n");
            
            // Group by file for better readability
            let mut current_file = String::new();
            for (path, name, kind, line) in results {
                if path != current_file {
                    current_file = path.clone();
                    output.push_str(&format!("File: {}\n", current_file));
                }
                output.push_str(&format!("  L{:<4} [{}] {}\n", line, kind, name));
            }

            Ok(ToolResult::ok(
                format!("Found {} matching symbols", output.lines().count() - 2), // approx count
                output
            ))
        }
        Err(e) => Ok(ToolResult::error(format!("Search failed: {}", e))),
    }
}

pub async fn execute_index_file(arguments: &str, working_dir: &Path) -> Result<ToolResult> {
    let args: IndexFileArgs = serde_json::from_str(arguments)?;
    let path = normalize_path(&args.path, working_dir);
    
    if !path.exists() {
        return Ok(ToolResult::error(format!("File not found: {}", path.display())));
    }

    let indexer = Indexer::new(working_dir)?;
    match indexer.index_file(&path, working_dir) {
        Ok(_) => Ok(ToolResult::ok(
            format!("Successfully indexed {}", path.display()),
            format!("Updated outline index for {}", path.display())
        )),
        Err(e) => Ok(ToolResult::error(format!("Failed to index file: {}", e))),
    }
}

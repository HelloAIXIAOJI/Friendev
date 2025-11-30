use anyhow::Result;
use ignore::WalkBuilder;
use regex::RegexBuilder;
use std::fs;
use std::path::Path;

use super::file_common::normalize_path;
use crate::tools::args::FileSearchArgs;
use crate::types::ToolResult;

pub async fn execute_file_search(arguments: &str, working_dir: &Path) -> Result<ToolResult> {
    let args: FileSearchArgs = serde_json::from_str(arguments)?;
    
    let search_path_str = args.path.as_deref().unwrap_or(".");
    let root_path = normalize_path(search_path_str, working_dir);
    
    if !root_path.exists() || !root_path.is_dir() {
        return Ok(ToolResult::error(format!("Directory not found: {}", root_path.display())));
    }

    let regex = RegexBuilder::new(&args.pattern)
        .case_insensitive(args.ignore_case)
        .build()
        .map_err(|e| anyhow::anyhow!("Invalid regex pattern: {}", e))?;

    let mut walker = WalkBuilder::new(&root_path);
    
    if let Some(glob) = args.include {
        let mut overrides = ignore::overrides::OverrideBuilder::new(&root_path);
        overrides.add(&glob)?;
        walker.overrides(overrides.build()?);
    }

    let mut results = String::new();
    let mut match_count = 0;
    const MAX_MATCHES: usize = 2000;
    let mut file_count = 0;

    for result in walker.build() {
        match result {
            Ok(entry) => {
                if !entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                    continue;
                }

                let path = entry.path();
                // Skip if we can't read it or it's binary (simple check)
                if let Ok(content) = fs::read_to_string(path) {
                    let mut file_matches = Vec::new();
                    for (line_idx, line) in content.lines().enumerate() {
                        if regex.is_match(line) {
                            file_matches.push((line_idx + 1, line));
                            match_count += 1;
                        }
                        if match_count >= MAX_MATCHES {
                            break;
                        }
                    }

                    if !file_matches.is_empty() {
                        file_count += 1;
                        results.push_str(&format!("File: {}\n", path.display()));
                        for (line_num, line_content) in file_matches {
                            // Truncate very long lines
                            let display_line = if line_content.len() > 200 {
                                format!("{}...", &line_content[..200])
                            } else {
                                line_content.to_string()
                            };
                            results.push_str(&format!("  {}: {}\n", line_num, display_line.trim()));
                        }
                        results.push_str("---\n");
                    }
                }
            }
            Err(_) => continue,
        }
        if match_count >= MAX_MATCHES {
            results.push_str(&format!("\n(Results truncated: exceeded {} matches)\n", MAX_MATCHES));
            break;
        }
    }

    if results.is_empty() {
        Ok(ToolResult::ok(
            format!("No matches found for '{}'", args.pattern),
            "No matches found.".to_string()
        ))
    } else {
        let brief = format!("Found {} matches in {} files", match_count, file_count);
        Ok(ToolResult::ok(brief, results))
    }
}

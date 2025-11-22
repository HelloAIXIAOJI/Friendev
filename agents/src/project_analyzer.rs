use super::gitattributes::{is_binary_file, load_gitattributes};
use anyhow::Result;
use ignore::WalkBuilder;
use std::path::Path;

/// Analyze project structure respecting .gitignore and .gitattributes
pub fn analyze_project_structure(working_dir: &Path) -> Result<String> {
    let mut structure = String::new();
    structure.push_str(
        "Project directory structure (respecting .gitignore and .gitattributes):\n\n```\n",
    );

    // Load binary file markers from .gitattributes
    let binary_files = load_gitattributes(working_dir);

    // Use ignore crate to properly handle .gitignore
    let walker = WalkBuilder::new(working_dir)
        .git_ignore(true)
        .git_global(false)
        .git_exclude(false)
        .standard_filters(false) // Don't use basic filters, let ignore handle .git etc
        .build();

    let mut entries: Vec<_> = vec![];

    // Collect top-level files and directories
    for entry in walker.flatten() {
        let path = entry.path();

        // Skip the parent directory itself
        if path == working_dir {
            continue;
        }

        // Skip non-direct children (depth > 1)
        let relative_path = path.strip_prefix(working_dir).unwrap_or(path);
        if relative_path.components().count() > 1 {
            continue;
        }

        entries.push(path.to_path_buf());
    }

    entries.sort();

    for path in entries {
        let file_name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if path.is_dir() {
            structure.push_str(&format!("{}/ (directory)\n", file_name));
        } else {
            // Check if it's a binary file
            let is_binary = is_binary_file(&file_name, &binary_files);
            if is_binary {
                structure.push_str(&format!("{} (binary file)\n", file_name));
            } else {
                structure.push_str(&format!("{} (file)\n", file_name));
            }
        }
    }

    structure.push_str("```\n");
    Ok(structure)
}

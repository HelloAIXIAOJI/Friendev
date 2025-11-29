use anyhow::Result;
use std::fs;
use std::path::Path;

/// Load existing AGENTS.md file from project root
pub fn load_agents_md(working_dir: &Path) -> Result<Option<String>> {
    let agents_path = working_dir.join("AGENTS.md");

    if agents_path.exists() {
        let content = fs::read_to_string(&agents_path)?;
        Ok(Some(content))
    } else {
        Ok(None)
    }
}

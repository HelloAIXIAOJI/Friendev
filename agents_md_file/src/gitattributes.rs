use std::fs;
use std::path::Path;

/// Load binary file patterns from .gitattributes
pub fn load_gitattributes(working_dir: &Path) -> Vec<String> {
    let gitattributes_path = working_dir.join(".gitattributes");

    if let Ok(content) = fs::read_to_string(gitattributes_path) {
        content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                // Skip comments and empty lines
                if line.is_empty() || line.starts_with('#') {
                    return None;
                }
                // Find lines with "binary" attribute
                if line.contains("binary") {
                    // Extract pattern part (first field)
                    let pattern = line.split_whitespace().next()?.to_string();
                    return Some(pattern);
                }
                None
            })
            .collect()
    } else {
        Vec::new()
    }
}

/// Check if a file is binary based on patterns
pub fn is_binary_file(filename: &str, binary_patterns: &[String]) -> bool {
    for pattern in binary_patterns {
        // Simple pattern matching: support * wildcard
        let regex_pattern = pattern.replace(".", "\\.").replace("*", ".*");

        if let Ok(re) = regex::Regex::new(&regex_pattern) {
            if re.is_match(filename) {
                return true;
            }
        }
    }
    false
}

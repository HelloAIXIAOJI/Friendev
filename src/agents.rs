use anyhow::Result;
use std::fs;
use std::path::Path;
use ignore::WalkBuilder;

/// 从项目根目录加载 AGENTS.md 文件（如果存在）
pub fn load_agents_md(working_dir: &Path) -> Result<Option<String>> {
    let agents_path = working_dir.join("AGENTS.md");
    
    if agents_path.exists() {
        let content = fs::read_to_string(&agents_path)?;
        Ok(Some(content))
    } else {
        Ok(None)
    }
}

/// 生成用于 AI 分析项目并生成 AGENTS.md 的提示词
pub fn generate_agents_analysis_prompt(working_dir: &Path) -> Result<String> {
    let project_structure = analyze_project_structure(working_dir)?;
    
    Ok(format!(
r#"Analyze this project and generate a comprehensive AGENTS.md file.

## Project Structure

{}

## Your Task

Based on the above project structure, generate a complete AGENS.md file that follows the AGENS.md open standard.

### AGENTS.md Standard

AGENTS.md is a Markdown file placed in the project root directory. It provides AI agents with essential information about how to work with the project.

**Core Principles:**
- **Clarity**: Specific commands and paths, no vague language
- **Completeness**: Include all AI-essential information
- **Accuracy**: Version numbers, exact command syntax
- **Brevity**: Only include what AI needs, no user tutorials

### Recommended Sections (customize as needed)

1. **Overview** - What is this project? (1-2 sentences)
2. **Dev Environment** - Prerequisites, versions, setup steps
3. **Project Structure** - Directory layout and key components
4. **Build & Compilation** - Build commands, output locations
5. **Testing** - Test execution, test patterns, coverage
6. **Code Style & Standards** - Linting rules, formatting, conventions
7. **Running the Application** - Start command, environment variables, configuration
8. **API & Dependencies** - External dependencies, version constraints
9. **Troubleshooting** - Common issues and solutions
10. **Contributing** - Git workflow, commit message format, PR guidelines

### Writing Guidelines

- Use concrete commands: `npm test` not "run tests"
- Include versions: `Python 3.11+` not "Python 3"
- Keep paths consistent: use relative paths from project root
- Format code blocks with language: \`\`\`bash, \`\`\`python, etc.
- Avoid marketing: no "amazing", "best", "leading"
- Be specific: "Run `pytest tests/` -cov" not "run the test suite"

### DO NOT include

- Project history or contributor lists
- User-level tutorials or getting started guides
- Marketing content
- Vague instructions like "or" / "either"

### Output Format

Output ONLY the AGENTS.md content in a markdown code block:

\`\`\`markdown
# AGENTS.md

[Your generated content...]
\`\`\`

Do not include any explanation before or after the code block. The content will be parsed and saved to the file.
"#,
        project_structure
    ))
}

/// 分析项目结构 - 使用 ignore crate 正确处理 .gitignore
fn analyze_project_structure(working_dir: &Path) -> Result<String> {
    let mut structure = String::new();
    structure.push_str("Project directory structure (respecting .gitignore and .gitattributes):\n\n```\n");
    
    // 从 .gitattributes 读取二进制文件标记
    let binary_files = load_gitattributes(working_dir);
    
    // 使用 ignore crate 正确遇罩 .gitignore
    let walker = WalkBuilder::new(working_dir)
        .git_ignore(true)
        .git_global(false)
        .git_exclude(false)
        .standard_filters(false)  // 不使用基本过滤器，種略 ignore 处理 .git 等
        .build();
    
    let mut entries: Vec<_> = vec![];
    
    // 収集一级文件和目录
    for result in walker {
        if let Ok(entry) = result {
            let path = entry.path();
            
            // 跳过父目录本身
            if path == working_dir {
                continue;
            }
            
            // 跳过非直接子项（深度 > 1）
            let relative_path = path.strip_prefix(working_dir).unwrap_or(path);
            if relative_path.components().count() > 1 {
                continue;
            }
            
            entries.push(path.to_path_buf());
        }
    }
    
    entries.sort();
    
    for path in entries {
        let file_name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        
        if path.is_dir() {
            structure.push_str(&format!("{}/ (directory)\n", file_name));
        } else {
            // 检查是否是二进制文件
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


/// 从 .gitattributes 文件加载二进制文件模式
fn load_gitattributes(working_dir: &Path) -> Vec<String> {
    let gitattributes_path = working_dir.join(".gitattributes");
    
    if let Ok(content) = fs::read_to_string(gitattributes_path) {
        content
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                // 跳过模式注释
                if line.is_empty() || line.starts_with('#') {
                    return None;
                }
                // 查找包含 "binary" 属性的行
                if line.contains("binary") {
                    // 提取模式部分（第一个字段）
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

/// 检查一个文件是否是二进制文件
fn is_binary_file(filename: &str, binary_patterns: &[String]) -> bool {
    for pattern in binary_patterns {
        // 简单的模式匹配：支持 * 通配符
        let regex_pattern = pattern
            .replace(".", "\\.")
            .replace("*", ".*");
        
        if let Ok(re) = regex::Regex::new(&regex_pattern) {
            if re.is_match(filename) {
                return true;
            }
        }
    }
    false
}

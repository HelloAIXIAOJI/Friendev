use anyhow::Result;
use std::path::Path;
use super::project_analyzer::analyze_project_structure;

/// Generate prompt for AI to analyze project and generate AGENTS.md
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
- Format code blocks with language: ```bash, ```python, etc.
- Avoid marketing: no "amazing", "best", "leading"
- Be specific: "Run `pytest tests/` -cov" not "run the test suite"

### DO NOT include

- Project history or contributor lists
- User-level tutorials or getting started guides
- Marketing content
- Vague instructions like "or" / "either"

### Output Format

Output ONLY the AGENTS.md content in a markdown code block:

```markdown
# AGENTS.md

[Your generated content...]
```

Do not include any explanation before or after the code block. The content will be parsed and saved to the file.
"#,
        project_structure
    ))
}

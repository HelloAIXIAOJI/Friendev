use super::project_analyzer::analyze_project_structure;
use anyhow::Result;
use std::path::Path;

/// Generate prompt for AI to analyze project and generate AGENTS.md
pub fn generate_agents_analysis_prompt(working_dir: &Path) -> Result<String> {
    let project_structure = analyze_project_structure(working_dir)?;

    // Load the AGENTS.md generator prompt template
    let template = include_str!("../../prompts/src/agents_generator_prompt.md");
    
    // Replace the {project_structure} placeholder
    let prompt = template.replace("{project_structure}", &project_structure);
    
    Ok(prompt)
}

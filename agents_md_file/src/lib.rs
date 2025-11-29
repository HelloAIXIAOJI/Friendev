mod gitattributes;
mod loader;
mod project_analyzer;
mod prompt_generator;

// Re-export public API
pub use loader::load_agents_md;
pub use prompt_generator::generate_agents_analysis_prompt;

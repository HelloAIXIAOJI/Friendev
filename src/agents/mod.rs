mod loader;
mod prompt_generator;
mod project_analyzer;
mod gitattributes;

// Re-export public API
pub use loader::load_agents_md;
pub use prompt_generator::generate_agents_analysis_prompt;

pub mod tools;

pub use tools::types;
pub use tools::{
    definitions::{get_available_tools, get_available_tools_with_mcp}, execute_tool, get_tools_description, get_tools_description_with_mcp, CommandConfig, Tool,
    ToolFunction, ToolResult,
};

// Re-export MCP-enabled executor functions
pub use tools::executor::execute_tool_with_mcp;

pub mod hooks;
pub mod tools;

pub use hooks::{HookType, execute_hook, HookContext};

pub use tools::{
    execute_tool,
    get_available_tools,
    get_available_tools_with_mcp,  // 来自feat分支
    get_tools_description,
    get_tools_description_with_mcp,  // 来自feat分支
    definitions::{CommandConfig, Tool, ToolFunction, ToolResult},
};

// Re-export MCP-enabled executor functions
pub use tools::executor::execute_tool_with_mcp;
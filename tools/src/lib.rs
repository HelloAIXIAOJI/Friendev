pub mod hooks;
pub mod tools;

pub use hooks::{HookType, execute_hook, HookContext};
pub use tools::{
    execute_tool, get_available_tools, get_tools_description, CommandConfig, Tool, ToolFunction,
    ToolResult,
};

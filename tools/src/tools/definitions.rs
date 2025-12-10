use super::{Tool, ToolFunction};
use serde_json::json;
use mcp::McpIntegration;

pub fn get_available_tools() -> Vec<Tool> {
    get_builtin_tools()
}

pub fn get_available_tools_with_mcp(mcp_integration: Option<&McpIntegration>) -> Vec<Tool> {
    let mut tools = get_builtin_tools();
    
    // Add MCP tools if integration is available  
    if let Some(integration) = mcp_integration {
        // MCP is initialized, add resource tools
        tools.extend(get_mcp_resource_tools());
        
        // Add dynamic MCP server tools via async call (need to block here or change architecture)
        // Since this function is sync, we use a block_in_place or runtime handle if available
        // Or we use the sync/async bridge.
        // For simplicity in this context, we assume we can block or the caller handles async.
        // However, this function is sync and called from sync contexts.
        // We'll use tokio::task::block_in_place if inside a runtime, or handle.block_on
        
        // SAFETY: This is a hack to call async from sync. In a real app, propagate async up.
        // But for now, we'll try to get a handle.
        if let Ok(handle) = tokio::runtime::Handle::try_current() {
             let mcp_tools = std::thread::scope(|s| {
                s.spawn(|| {
                    handle.block_on(integration.get_server_tools_definitions())
                }).join().unwrap_or_default()
            });
            
            for tool_def in mcp_tools {
                tools.push(Tool {
                    tool_type: "function".to_string(),
                    function: ToolFunction {
                        name: tool_def.name,
                        description: tool_def.description,
                        parameters: tool_def.parameters,
                    },
                });
            }
        }
    }
    
    tools
}

pub fn get_builtin_tools() -> Vec<Tool> {
    vec![
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_list".to_string(),
                description: "List all files and subdirectories in the specified directory".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Directory path (optional, defaults to working directory)"
                        }
                    },
                    "required": []
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_read".to_string(),
                description: "Read the content of a file. Supports optional line range reading (1-indexed).".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "File path to read"
                        },
                        "start_line": {
                            "type": "integer",
                            "description": "Start line number (optional, 1-indexed)"
                        },
                        "end_line": {
                            "type": "integer",
                            "description": "End line number (optional, 1-indexed)"
                        }
                    },
                    "required": ["path"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_write".to_string(),
                description: "Write content to a file.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "File path to write"
                        },
                        "content": {
                            "type": "string",
                            "description": "Content to write"
                        },
                        "mode": {
                            "type": "string",
                            "enum": ["overwrite", "append"],
                            "description": "Write mode: 'overwrite' to replace file content (default), 'append' to add to end of file",
                            "default": "overwrite"
                        }
                    },
                    "required": ["path", "content"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_replace".to_string(),
                description: "Replace strings in a file, supporting batch edits. Prefer this tool over file_write to modify existing files.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "File path to edit"
                        },
                        "edits": {
                            "type": "array",
                            "description": "List of edit operations to apply in order",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "old": {
                                        "type": "string",
                                        "description": "Old string to replace (supports multi-line)"
                                    },
                                    "new": {
                                        "type": "string",
                                        "description": "New string (supports multi-line)"
                                    },
                                    "replace_all": {
                                        "type": "boolean",
                                        "description": "Whether to replace all matches (default false, replaces only the first)",
                                        "default": false
                                    },
                                    "normalize": {
                                        "type": "boolean",
                                        "description": "If true, uses loose matching: ignores leading/trailing whitespace and normalizes line endings (default false for exact match)",
                                        "default": false
                                    },
                                    "regex": {
                                        "type": "boolean",
                                        "description": "If true, treats 'old' as a regular expression pattern for flexible matching (e.g., pattern.*content, \\d+ for numbers)",
                                        "default": false
                                    }
                                },
                                "required": ["old", "new"]
                            }
                        }
                    },
                    "required": ["path", "edits"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_search".to_string(),
                description: "Search for a pattern in files recursively, respecting .gitignore. Similar to grep/ripgrep.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "pattern": {
                            "type": "string",
                            "description": "Regex pattern to search for"
                        },
                        "path": {
                            "type": "string",
                            "description": "Root directory to search in (defaults to current directory)"
                        },
                        "include": {
                            "type": "string",
                            "description": "Glob pattern to include files (e.g., '*.rs')"
                        },
                        "ignore_case": {
                            "type": "boolean",
                            "description": "Case insensitive search",
                            "default": false
                        }
                    },
                    "required": ["pattern"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_outline".to_string(),
                description: "Extract symbol definitions (functions, classes, structs, etc.) from a file using Tree-sitter. Supports Rust, Python, JS/TS, Go, Java, C/C++, C#, PHP, Ruby.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to outline"
                        }
                    },
                    "required": ["path"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_search_by_outline".to_string(),
                description: "Search for symbol definitions in the local database. Fast but results depend on index freshness. Use /index outline to update.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "pattern": {
                            "type": "string",
                            "description": "Pattern to search for (SQL LIKE syntax, e.g., 'process%')"
                        }
                    },
                    "required": ["pattern"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "index_file".to_string(),
                description: "Update the outline index for a specific file. Use this after creating new files to keep the index fresh.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "Path to the file to index"
                        }
                    },
                    "required": ["path"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "network_search_auto".to_string(),
                description: "Search the web with automatic fallback: tries DuckDuckGo first, then Bing if DuckDuckGo fails. Returns title, URL, and snippet for each result.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "keywords": {
                            "type": "string",
                            "description": "Search keywords or query"
                        },
                        "max_results": {
                            "type": "integer",
                            "description": "Maximum number of results to return (default 5, max 20)",
                            "default": 5,
                            "minimum": 1,
                            "maximum": 20
                        }
                    },
                    "required": ["keywords"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "network_search_duckduckgo".to_string(),
                description: "Search the web using DuckDuckGo search engine. Returns title, URL, and snippet for each result.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "keywords": {
                            "type": "string",
                            "description": "Search keywords or query"
                        },
                        "max_results": {
                            "type": "integer",
                            "description": "Maximum number of results to return (default 5, max 20)",
                            "default": 5,
                            "minimum": 1,
                            "maximum": 20
                        }
                    },
                    "required": ["keywords"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "network_search_bing".to_string(),
                description: "Search the web using Bing search engine. Returns title, URL, and snippet for each result.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "keywords": {
                            "type": "string",
                            "description": "Search keywords or query"
                        },
                        "max_results": {
                            "type": "integer",
                            "description": "Maximum number of results to return (default 5, max 20)",
                            "default": 5,
                            "minimum": 1,
                            "maximum": 20
                        }
                    },
                    "required": ["keywords"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "network_get_content".to_string(),
                description: "Fetch textual content from a URL via HTTP GET with size and content-type safeguards.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "url": {
                            "type": "string",
                            "description": "HTTP or HTTPS URL to fetch"
                        },
                        "max_bytes": {
                            "type": "integer",
                            "description": "Optional maximum number of bytes to read (defaults to 524288, min 1024, max 1048576)",
                            "minimum": 1024,
                            "maximum": 1048576
                        }
                    },
                    "required": ["url"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "file_diff_edit".to_string(),
                description: "Edit file content using diff-style hunks. Each hunk specifies a line range and its new content. This is useful for precise multi-location edits.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "File path to edit"
                        },
                        "hunks": {
                            "type": "array",
                            "description": "List of diff hunks to apply in order",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "start_line": {
                                        "type": "integer",
                                        "description": "Starting line number (1-indexed)"
                                    },
                                    "num_lines": {
                                        "type": "integer",
                                        "description": "Number of lines to replace in the original file"
                                    },
                                    "new_content": {
                                        "type": "string",
                                        "description": "New content to replace the old lines (multi-line supported)"
                                    }
                                },
                                "required": ["start_line", "num_lines", "new_content"]
                            }
                        }
                    },
                    "required": ["path", "hunks"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "run_command".to_string(),
                description: "Execute a shell command with approval prompts. Supports foreground and background execution.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "command": {
                            "type": "string",
                            "description": "The shell command to execute"
                        },
                        "background": {
                            "type": "boolean",
                            "description": "Whether to run the command in background (returns immediately with a run_id) or foreground (waits and returns output)",
                            "default": false
                        }
                    },
                    "required": ["command"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "todo_write".to_string(),
                description: "Creates and manages a structured task list for the current coding session. Helps track progress and organize complex tasks.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "todos": {
                            "type": "array",
                            "description": "The updated todo list",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "content": {
                                        "type": "string",
                                        "description": "The task description",
                                        "minLength": 1
                                    },
                                    "status": {
                                        "type": "string",
                                        "enum": ["pending", "in_progress", "completed"],
                                        "description": "Current status of the task"
                                    },
                                    "priority": {
                                        "type": "string",
                                        "enum": ["high", "medium", "low"],
                                        "description": "Priority level of the task"
                                    },
                                    "id": {
                                        "type": "string",
                                        "description": "Unique identifier for the task"
                                    }
                                },
                                "required": ["content", "status", "id"],
                                "additionalProperties": false
                            }
                        }
                    },
                    "required": ["todos"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "todo_read".to_string(),
                description: "Read the current todo list to check progress and pending tasks.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "task".to_string(),
                description: "Delegate a complex task to a specialized subagent. The subagent runs in a separate session with its own context.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "description": {
                            "type": "string",
                            "description": "A short description of the task (3-5 words)"
                        },
                        "prompt": {
                            "type": "string",
                            "description": "Detailed instructions for the subagent. Include all necessary context as the subagent starts with a fresh state."
                        },
                        "subagent_type": {
                            "type": "string",
                            "description": "Type of subagent to use (e.g., 'general', 'coder', 'reviewer'). Defaults to 'general'.",
                            "default": "general"
                        }
                    },
                    "required": ["description", "prompt"]
                }),
            },
        },
    ]
}

pub fn get_mcp_resource_tools() -> Vec<Tool> {
    vec![
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "mcp_resource_read".to_string(),
                description: "Read content from an MCP resource URI. Supports various resource types like text://, file://, memory://, etc.".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "resource_uri": {
                            "type": "string",
                            "description": "The MCP resource URI to read (e.g., text://hello, file:///path/to/file, memory://key)"
                        },
                        "mcp_server": {
                            "type": "string",
                            "description": "Optional MCP server name. If not specified, searches all connected servers"
                        }
                    },
                    "required": ["resource_uri"]
                }),
            },
        },
        Tool {
            tool_type: "function".to_string(),
            function: ToolFunction {
                name: "mcp_resource_list".to_string(),
                description: "List all available MCP resources from connected servers".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "mcp_server": {
                            "type": "string",
                            "description": "Optional MCP server name. If not specified, lists resources from all connected servers"
                        }
                    },
                    "required": []
                }),
            },
        },
    ]
}

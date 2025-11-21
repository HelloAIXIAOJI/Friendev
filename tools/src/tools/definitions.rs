use crate::tools::{Tool, ToolFunction};
use serde_json::json;

pub fn get_available_tools() -> Vec<Tool> {
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
                description: "Read the content of a file".to_string(),
                parameters: json!({
                    "type": "object",
                    "properties": {
                        "path": {
                            "type": "string",
                            "description": "File path to read"
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
    ]
}

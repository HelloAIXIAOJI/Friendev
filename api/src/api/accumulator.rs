use crate::history::ToolCall;
use crate::ui::get_i18n;
use crate::ui::ToolCallDisplay;

use super::parser::is_json_semantically_complete;

pub struct ToolCallAccumulator {
    calls: std::collections::HashMap<String, (String, String)>,
    last_id: Option<String>,
    displays: std::collections::HashMap<String, ToolCallDisplay>,
    has_tool_calls: bool,
    has_finish_reason: bool,
    finish_reason: Option<String>,
}

impl ToolCallAccumulator {
    pub fn new() -> Self {
        Self {
            calls: std::collections::HashMap::new(),
            last_id: None,
            displays: std::collections::HashMap::new(),
            has_tool_calls: false,
            has_finish_reason: false,
            finish_reason: None,
        }
    }

    /// Record finish_reason
    pub fn set_finish_reason(&mut self, reason: String) {
        self.has_finish_reason = true;
        self.finish_reason = Some(reason);
    }

    pub fn add_chunk(&mut self, id: String, name: String, arguments: String) {
        // Mark that tool calls were detected
        if !id.is_empty() || !name.is_empty() || !arguments.is_empty() {
            self.has_tool_calls = true;
        }

        // Use last valid ID if current ID is empty
        let key = if id.is_empty() {
            self.last_id.clone().unwrap_or_else(|| "temp".to_string())
        } else {
            self.last_id = Some(id.clone());
            id.clone()
        };

        let entry = self
            .calls
            .entry(key.clone())
            .or_insert((String::new(), String::new()));

        // Update name if provided
        if !name.is_empty() {
            entry.0 = name.clone();
            // Create UI display component
            if !self.displays.contains_key(&key) {
                self.displays
                    .insert(key.clone(), ToolCallDisplay::new(name.clone()));
            }
        }

        // Append arguments
        if !arguments.is_empty() {
            entry.1.push_str(&arguments);

            // Try to extract key argument and update UI
            if let Some(display) = self.displays.get_mut(&key) {
                let tool_name = &entry.0;
                if let Some(arg) = crate::ui::extract_key_argument(tool_name, &entry.1) {
                    display.update_argument(arg);
                }
                display.render_streaming();
            }
        }
    }

    /// Get all UI display components
    pub fn get_displays(&self) -> &std::collections::HashMap<String, ToolCallDisplay> {
        &self.displays
    }

    pub fn into_tool_calls(self) -> Vec<ToolCall> {
        let has_tool_calls = self.has_tool_calls;

        self.calls
            .into_iter()
            .filter_map(|(id, (name, arguments))| {
                // Filter out empty tool calls
                if name.is_empty() || arguments.is_empty() {
                    let i18n = get_i18n();
                    eprintln!(
                        "\x1b[33m[!] {}:\x1b[0m {} id={}",
                        i18n.get("warning"),
                        i18n.get("api_skip_empty_tool_call"),
                        id
                    );
                    return None;
                }

                // Validate JSON is semantically complete
                if !is_json_semantically_complete(&name, &arguments) {
                    let preview: String = arguments.chars().take(50).collect();
                    let i18n = get_i18n();
                    eprintln!(
                        "\x1b[33m[!] {}:\x1b[0m {} '{}': {}",
                        i18n.get("warning"),
                        i18n.get("api_incomplete_json"),
                        name,
                        preview
                    );
                    return None;
                }

                // Validate and fix JSON if needed
                let fixed_arguments =
                    if serde_json::from_str::<serde_json::Value>(&arguments).is_err() {
                        let mut fixed = arguments.clone();

                        // Special handling for file_write content truncation
                        if name == "file_write" && has_tool_calls {
                            if let Some(content_start) = fixed.rfind(r#""content""#) {
                                let after_content = &fixed[content_start..];
                                if after_content.matches('"').count() % 2 != 0 {
                                    fixed.push('"');
                                }
                            }
                        }

                        // 1. Add missing closing braces
                        let open_braces = fixed.matches('{').count();
                        let close_braces = fixed.matches('}').count();
                        if open_braces > close_braces {
                            for _ in 0..(open_braces - close_braces) {
                                fixed.push('}');
                            }
                        }

                        // 2. Add missing quotes (global check)
                        if fixed.matches('"').count() % 2 != 0 {
                            fixed.push('"');
                        }

                        // 3. Validate fixed JSON
                        if serde_json::from_str::<serde_json::Value>(&fixed).is_ok() {
                            let i18n = get_i18n();
                            eprintln!(
                                "\x1b[32m[✓] {}:\x1b[0m {} '{}' (has_tool_calls={})",
                                i18n.get("info"),
                                i18n.get("api_auto_fixed_json"),
                                name,
                                has_tool_calls
                            );
                            fixed
                        } else {
                            let i18n = get_i18n();
                            eprintln!(
                                "\x1b[31m[✗] {}:\x1b[0m {} '{}'",
                                i18n.get("error"),
                                i18n.get("api_failed_fix_json"),
                                name
                            );
                            return None;
                        }
                    } else {
                        arguments.clone()
                    };

                Some(ToolCall {
                    id,
                    tool_type: "function".to_string(),
                    function: crate::history::FunctionCall {
                        name,
                        arguments: fixed_arguments,
                    },
                })
            })
            .collect()
    }
}

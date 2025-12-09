use std::collections::HashMap;

pub fn get_messages() -> HashMap<String, String> {
    let mut m = HashMap::new();

    // Startup messages
    m.insert("config_loaded".to_string(), "Config loaded".to_string());
    m.insert("working_dir".to_string(), "Working directory".to_string());
    m.insert("new_session".to_string(), "New session".to_string());
    m.insert(
        "welcome_subtitle".to_string(),
        "AI-Powered Development Assistant".to_string(),
    );
    m.insert("current_model".to_string(), "Current Model".to_string());
    m.insert(
        "available_commands".to_string(),
        "Available Commands".to_string(),
    );
    m.insert(
        "type_message".to_string(),
        ">> Type a message to start chatting".to_string(),
    );

    // Commands
    m.insert(
        "cmd_model_interactive".to_string(),
        "Interactive model selector".to_string(),
    );
    m.insert("cmd_model_list".to_string(), "List all models".to_string());
    m.insert("cmd_model_switch".to_string(), "Switch model".to_string());
    m.insert(
        "cmd_history_list".to_string(),
        "List chat history".to_string(),
    );
    m.insert(
        "cmd_history_switch".to_string(),
        "Switch session".to_string(),
    );
    m.insert(
        "cmd_history_new".to_string(),
        "Create new session".to_string(),
    );
    m.insert("cmd_history_del".to_string(), "Delete session".to_string());
    m.insert("cmd_language_ui".to_string(), "Set UI language".to_string());
    m.insert("cmd_language_ai".to_string(), "Set AI language".to_string());
    m.insert("cmd_help".to_string(), "Show help".to_string());
    m.insert("cmd_exit".to_string(), "Exit program".to_string());
    m.insert(
        "cmd_agents_md".to_string(),
        "Generate AGENTS.md file".to_string(),
    );
    m.insert(
        "cmd_runcommand_list".to_string(),
        "List commands requiring approval".to_string(),
    );
    m.insert(
        "cmd_runcommand_add".to_string(),
        "Add command to approval list".to_string(),
    );
    m.insert(
        "cmd_runcommand_del".to_string(),
        "Remove command from approval list".to_string(),
    );
    m.insert(
        "cmd_runcommand_info".to_string(),
        "Show background command details".to_string(),
    );

    // Status messages
    m.insert("goodbye".to_string(), "Goodbye!".to_string());
    m.insert(
        "loading_models".to_string(),
        "Loading models...".to_string(),
    );
    m.insert(
        "available_models".to_string(),
        "Available Models".to_string(),
    );
    m.insert(
        "switched_model".to_string(),
        "Switched to model".to_string(),
    );
    m.insert(
        "model_selector_prompt".to_string(),
        "Select a model".to_string(),
    );
    m.insert(
        "model_already_active".to_string(),
        "Model already active:".to_string(),
    );
    m.insert(
        "model_selection_cancelled".to_string(),
        "Model selection cancelled".to_string(),
    );
    m.insert(
        "no_models_available".to_string(),
        "No models available".to_string(),
    );
    m.insert(
        "interactive_mode_failed".to_string(),
        "Interactive mode not available, showing list".to_string(),
    );
    m.insert(
        "switched_session".to_string(),
        "Switched to session".to_string(),
    );
    m.insert(
        "created_session".to_string(),
        "Created new session".to_string(),
    );
    m.insert("deleted_session".to_string(), "Deleted session".to_string());
    m.insert("no_history".to_string(), "No chat history".to_string());
    m.insert("chat_history".to_string(), "Chat History".to_string());
    m.insert("messages".to_string(), "msgs".to_string());
    m.insert("history_menu_title".to_string(), "Select a session".to_string());
    m.insert("history_new_session".to_string(), "New Session".to_string());
    m.insert("history_current_session".to_string(), "(Current)".to_string());
    m.insert("history_empty".to_string(), "No history found in this directory".to_string());
    m.insert("session_already_active".to_string(), "Session already active".to_string());

    // Tool messages
    m.insert("tool_call".to_string(), "TOOL".to_string());
    m.insert("thinking".to_string(), "THINK".to_string());
    m.insert("tools_header".to_string(), "Using Tools".to_string());

    // Hint messages
    m.insert("hint_send".to_string(), "Send".to_string());
    m.insert("hint_newline".to_string(), "New Line".to_string());
    m.insert(
        "hint_short".to_string(),
        "Enter = Send  |  Alt+Enter = New Line".to_string(),
    );
    m.insert(
        "hint_shift_enter".to_string(),
        "Shift+Enter or ! prefix = Optimize Prompt".to_string(),
    );
    m.insert(
        "hint_esc".to_string(),
        "ESC = Stop Generation".to_string(),
    );
    m.insert(
        "hint_ctrl_c_twice".to_string(),
        "Press Ctrl+C again to exit".to_string(),
    );
    m.insert(
        "hint_ctrl_c_exit".to_string(),
        "Ctrl+C twice = Exit".to_string(),
    );

    // Error messages
    m.insert("error".to_string(), "Error".to_string());
    m.insert("api_error".to_string(), "API Error".to_string());
    m.insert("unknown_command".to_string(), "Unknown command".to_string());
    m.insert("usage".to_string(), "Usage".to_string());
    m.insert(
        "failed_load_models".to_string(),
        "Failed to load models".to_string(),
    );
    m.insert(
        "failed_load_session".to_string(),
        "Failed to load session".to_string(),
    );
    m.insert("invalid_uuid".to_string(), "Invalid UUID".to_string());
    m.insert(
        "cannot_delete_current".to_string(),
        "Cannot delete current session".to_string(),
    );

    // Help
    m.insert(
        "help_title".to_string(),
        "Help - Available Commands".to_string(),
    );
    m.insert("help_model".to_string(), "Model Commands".to_string());
    m.insert("help_history".to_string(), "History Commands".to_string());
    m.insert("help_language".to_string(), "Language Commands".to_string());
    m.insert("help_other".to_string(), "Other Commands".to_string());
    m.insert(
        "help_runcommand".to_string(),
        "Run Command Settings".to_string(),
    );

    // Language
    m.insert(
        "ui_language_set".to_string(),
        "UI language set to".to_string(),
    );
    m.insert(
        "ai_language_set".to_string(),
        "AI response language set to".to_string(),
    );
    m.insert(
        "supported_languages".to_string(),
        "AI support depends on the model.".to_string(),
    );
    m.insert(
        "current_ui_lang".to_string(),
        "Current UI Language".to_string(),
    );
    m.insert(
        "current_ai_lang".to_string(),
        "Current AI Language".to_string(),
    );

    // Setup initialization
    m.insert(
        "setup_welcome".to_string(),
        "Welcome to Friendev! First-time use requires initialization configuration.".to_string(),
    );
    m.insert(
        "setup_api_key".to_string(),
        "Please enter OpenAI API Key".to_string(),
    );
    m.insert(
        "setup_api_url".to_string(),
        "Please enter OpenAI Base URL".to_string(),
    );
    m.insert(
        "setup_model".to_string(),
        "Please enter the default model".to_string(),
    );
    m.insert(
        "setup_ui_language".to_string(),
        "Please select UI language".to_string(),
    );
    m.insert(
        "setup_ai_language".to_string(),
        "Please enter AI response language".to_string(),
    );
    m.insert(
        "setup_saved".to_string(),
        "Configuration saved!".to_string(),
    );

    // Approval UI
    m.insert("approval_title".to_string(), "Approval Required".to_string());
    m.insert("approval_action_wants".to_string(), "wants to execute".to_string());
    m.insert("approval_content_preview".to_string(), "Content Preview".to_string());
    m.insert("approval_always_approved".to_string(), "Always approved for this session".to_string());
    m.insert("approval_rejected".to_string(), "Operation rejected".to_string());
    m.insert("approval_opt_approve".to_string(), "Approve".to_string());
    m.insert("approval_opt_always".to_string(), "Approve Always (Session)".to_string());
    m.insert("approval_opt_details".to_string(), "View Details / Review".to_string());
    m.insert("approval_opt_reject".to_string(), "Reject".to_string());

    m.insert(
        "approval_review_unavailable".to_string(),
        "Review helper is not available.".to_string(),
    );
    m.insert(
        "approval_review_error".to_string(),
        "Review failed: {}".to_string(),
    );
    m.insert(
        "approval_review_request".to_string(),
        "Requesting AI review for action '{}'.".to_string(),
    );
    m.insert(
        "approval_review_wait".to_string(),
        "Waiting for review response...".to_string(),
    );
    m.insert(
        "approval_review_done".to_string(),
        "Review completed".to_string(),
    );
    m.insert(
        "approval_review_result".to_string(),
        "AI Review:".to_string(),
    );
    m.insert(
        "approval_review_tool_error".to_string(),
        "Review returned unsupported tool calls.".to_string(),
    );
    m.insert(
        "approval_review_no_preview".to_string(),
        "(no additional preview available)".to_string(),
    );
    m.insert(
        "approval_review_parse_error".to_string(),
        "Failed to parse review output: {}".to_string(),
    );
    m.insert(
        "approval_review_raw".to_string(),
        "Raw response:".to_string(),
    );
    m.insert(
        "approval_review_decision".to_string(),
        "Decision:".to_string(),
    );
    m.insert(
        "approval_review_details".to_string(),
        "Details:".to_string(),
    );
    m.insert(
        "approval_review_followup".to_string(),
        "Review complete. Please enter the final decision (Y/N only).".to_string(),
    );
    m.insert(
        "approval_review_decision_prompt".to_string(),
        "Final decision [Y/N]:".to_string(),
    );
    m.insert(
        "approval_review_invalid_choice".to_string(),
        "Invalid choice. Please enter Y or N.".to_string(),
    );
    m.insert(
        "approval_review_decision_yes".to_string(),
        "Approve".to_string(),
    );
    m.insert(
        "approval_review_decision_no".to_string(),
        "Reject".to_string(),
    );

    m.insert(
        "details_title".to_string(),
        "  ──── Detailed Code Changes ──────────────────".to_string(),
    );
    m.insert(
        "details_separator".to_string(),
        "  ──────────────────────────────────────────".to_string(),
    );
    m.insert("details_tool".to_string(), "Tool:".to_string());
    m.insert("details_file".to_string(), "File:".to_string());
    m.insert(
        "details_choice_hint".to_string(),
        "[C]ontinue / [A]bort".to_string(),
    );
    m.insert(
        "details_choice_prompt".to_string(),
        "Your choice:".to_string(),
    );

    // UI: tool call display
    m.insert("tool_action_used".to_string(), "Used".to_string());
    m.insert("tool_action_using".to_string(), "Using".to_string());

    // Tools & executor messages
    m.insert("tool_unknown".to_string(), "Unknown tool: {}".to_string());

    m.insert(
        "file_not_exist".to_string(),
        "File does not exist: {}".to_string(),
    );
    m.insert("file_not_file".to_string(), "Not a file: {}".to_string());
    m.insert(
        "file_path_not_exist".to_string(),
        "Path does not exist: {}".to_string(),
    );
    m.insert(
        "file_not_directory".to_string(),
        "Not a directory: {}".to_string(),
    );

    m.insert("file_item_type_dir".to_string(), "DIR".to_string());
    m.insert("file_item_type_file".to_string(), "FILE".to_string());
    m.insert("file_item_size_na".to_string(), "-".to_string());
    m.insert("file_list_item".to_string(), "{} [{}] ({})".to_string());
    m.insert(
        "file_list_empty".to_string(),
        "Directory is empty".to_string(),
    );
    m.insert("file_list_brief".to_string(), "Listed {} items".to_string());
    m.insert("file_list_header".to_string(), "Directory: {}".to_string());
    m.insert("file_list_count".to_string(), "Total: {} items".to_string());

    m.insert(
        "file_read_brief".to_string(),
        "Read {} lines, {} bytes".to_string(),
    );
    m.insert(
        "file_read_header".to_string(),
        "File: {}\nContent:".to_string(),
    );

    m.insert(
        "file_write_invalid_mode".to_string(),
        "Invalid write mode: {}, only 'overwrite' or 'append' are supported".to_string(),
    );
    m.insert(
        "file_write_append_action".to_string(),
        "Append to file: {}".to_string(),
    );
    m.insert(
        "file_write_overwrite_action".to_string(),
        "Overwrite file: {}".to_string(),
    );
    m.insert(
        "file_write_append_brief".to_string(),
        "Appended {} bytes".to_string(),
    );
    m.insert(
        "file_write_append_output".to_string(),
        "Successfully appended to file: {}\nAppended: {} bytes\nCurrent size: {} bytes".to_string(),
    );
    m.insert(
        "file_write_overwrite_brief".to_string(),
        "Wrote {} bytes".to_string(),
    );
    m.insert(
        "file_write_overwrite_output".to_string(),
        "Successfully wrote file: {}\nSize: {} bytes".to_string(),
    );

    // Search tool messages
    m.insert(
        "search_engine_prefix".to_string(),
        "Search engine: {}\n".to_string(),
    );
    m.insert("search_keywords_label".to_string(), "Keywords".to_string());
    m.insert("search_found_label".to_string(), "Found".to_string());
    m.insert("search_url_label".to_string(), "URL".to_string());
    m.insert("search_snippet_label".to_string(), "Snippet".to_string());
    m.insert(
        "search_brief_with_engine".to_string(),
        "{}: found {} results".to_string(),
    );
    m.insert("search_brief".to_string(), "Found {} results".to_string());
    m.insert(
        "search_error_with_engine".to_string(),
        "{} search failed: {}".to_string(),
    );
    m.insert("search_error".to_string(), "Search failed: {}".to_string());
    m.insert(
        "search_ddg_no_results".to_string(),
        "DuckDuckGo: no results found".to_string(),
    );
    m.insert(
        "search_bing_request_failed".to_string(),
        "Bing request failed".to_string(),
    );
    m.insert(
        "search_bing_status_code".to_string(),
        "Bing returned status code".to_string(),
    );
    m.insert(
        "search_bing_read_failed".to_string(),
        "Failed to read Bing response".to_string(),
    );
    m.insert(
        "search_bing_no_results".to_string(),
        "Bing: no results found".to_string(),
    );
    m.insert(
        "search_ddg_error_prefix".to_string(),
        "DuckDuckGo ERROR".to_string(),
    );
    m.insert("search_try_bing".to_string(), "Try Bing...".to_string());

    // Network fetch tool messages
    m.insert(
        "network_fetch_invalid_url".to_string(),
        "Invalid URL: {}".to_string(),
    );
    m.insert(
        "network_fetch_unsupported_scheme".to_string(),
        "Unsupported URL scheme: {} (only http and https are allowed)".to_string(),
    );
    m.insert(
        "network_fetch_request_error".to_string(),
        "Failed to fetch URL: {}".to_string(),
    );
    m.insert(
        "network_fetch_timeout".to_string(),
        "Request timed out while fetching the URL.".to_string(),
    );
    m.insert(
        "network_fetch_status_error".to_string(),
        "Request failed with status {} ({})".to_string(),
    );
    m.insert(
        "network_fetch_too_large".to_string(),
        "Response too large (limit {}).".to_string(),
    );
    m.insert(
        "network_fetch_non_text".to_string(),
        "Unsupported content type: {} (only textual responses are allowed).".to_string(),
    );
    m.insert(
        "network_fetch_brief".to_string(),
        "Fetched {} from URL.".to_string(),
    );
    m.insert(
        "network_fetch_brief_truncated".to_string(),
        "Fetched {} from URL (truncated).".to_string(),
    );
    m.insert(
        "network_fetch_truncated_note".to_string(),
        "Note: Content truncated to {}.".to_string(),
    );
    m.insert(
        "network_fetch_html_note".to_string(),
        "Note: HTML content converted to plain text.".to_string(),
    );
    m.insert(
        "network_fetch_output".to_string(),
        "URL: {}\nStatus: {}\nContent-Type: {}\nSize: {}\n{}\nContent:\n{}".to_string(),
    );

    // Run command tool messages
    m.insert(
        "run_command_user_cancelled".to_string(),
        "User cancelled the operation".to_string(),
    );
    m.insert(
        "run_command_user_rejected".to_string(),
        "User rejected the operation".to_string(),
    );
    m.insert(
        "run_command_bg_brief".to_string(),
        "Started background command: {}".to_string(),
    );
    m.insert("run_command_bg_output".to_string(), "Command started in background\nRun ID: {}\nCommand: {}\n\nUse /runcommand info {{}} to check status".to_string());
    m.insert(
        "run_command_fg_brief".to_string(),
        "Command executed: {} (exit: {})".to_string(),
    );
    m.insert(
        "run_command_fg_output".to_string(),
        "Command: {}\nExit code: {}\nStatus: {}\n\nOutput:\n{}".to_string(),
    );
    m.insert(
        "run_command_execute_error".to_string(),
        "Failed to execute command: {}".to_string(),
    );

    // Language command extras
    m.insert(
        "lang_ui_unsupported".to_string(),
        "Unsupported UI language: '{}'".to_string(),
    );
    m.insert(
        "lang_supported_label".to_string(),
        "Supported languages".to_string(),
    );
    m.insert(
        "lang_supported_ui_label".to_string(),
        "Supported UI languages:".to_string(),
    );

    // Runcommand command messages
    m.insert(
        "runcommand_no_commands".to_string(),
        "No commands require approval".to_string(),
    );
    m.insert(
        "runcommand_list_header".to_string(),
        "Commands requiring approval".to_string(),
    );
    m.insert(
        "runcommand_load_config_failed".to_string(),
        "Failed to load command config".to_string(),
    );
    m.insert(
        "runcommand_add_ok".to_string(),
        "Added '{}' to approval list".to_string(),
    );
    m.insert(
        "runcommand_add_exists".to_string(),
        "'{}' is already in approval list".to_string(),
    );
    m.insert(
        "runcommand_del_ok".to_string(),
        "Removed '{}' from approval list".to_string(),
    );
    m.insert(
        "runcommand_del_not_found".to_string(),
        "'{}' is not in approval list".to_string(),
    );
    m.insert(
        "runcommand_info_header".to_string(),
        "Background Command Info".to_string(),
    );
    m.insert("runcommand_info_id".to_string(), "ID:".to_string());
    m.insert(
        "runcommand_info_command".to_string(),
        "Command:".to_string(),
    );
    m.insert("runcommand_info_status".to_string(), "Status:".to_string());
    m.insert(
        "runcommand_info_started".to_string(),
        "Started:".to_string(),
    );
    m.insert(
        "runcommand_info_exit_code".to_string(),
        "Exit Code:".to_string(),
    );
    m.insert("runcommand_info_output".to_string(), "Output".to_string());
    m.insert(
        "runcommand_info_not_found".to_string(),
        "Command with ID '{}' not found".to_string(),
    );
    m.insert(
        "runcommand_help_header".to_string(),
        "Help for /runcommand".to_string(),
    );

    // Agents command messages
    m.insert(
        "agents_analyzing_project".to_string(),
        "Analyzing project structure...".to_string(),
    );
    m.insert(
        "agents_sending_to_ai".to_string(),
        "Sending to AI for AGENTS.md generation...".to_string(),
    );

    // History maintenance messages
    m.insert(
        "history_cleanup_empty".to_string(),
        "Cleaned up {} empty session(s)".to_string(),
    );

    // History summary
    m.insert(
        "history_new_chat_summary".to_string(),
        "New Chat".to_string(),
    );

    // Chat output labels
    m.insert("chat_think_label".to_string(), "THINK".to_string());
    m.insert("chat_ai_label".to_string(), "AI".to_string());
    m.insert(
        "chat_tool_parse_error".to_string(),
        "Tool calls detected but all failed to parse".to_string(),
    );
    m.insert(
        "chat_debug_info_label".to_string(),
        "Debug Info".to_string(),
    );
    m.insert(
        "chat_tool_parse_debug".to_string(),
        "Check if tool arguments are valid JSON".to_string(),
    );

    // Security messages
    m.insert(
        "security_warning_label".to_string(),
        "Security Warning".to_string(),
    );
    m.insert(
        "security_forbidden_tokens".to_string(),
        "Input contains forbidden control tokens".to_string(),
    );

    // API messages
    m.insert("api_retry_label".to_string(), "Retry".to_string());
    m.insert("api_retry_waiting".to_string(), "waiting".to_string());
    m.insert(
        "api_retries_failed".to_string(),
        "All retries failed".to_string(),
    );
    m.insert(
        "api_request_failed".to_string(),
        "Request failed".to_string(),
    );
    m.insert(
        "api_models_failed".to_string(),
        "Failed to fetch models list".to_string(),
    );
    m.insert(
        "api_stream_error".to_string(),
        "Stream error: {}".to_string(),
    );
    m.insert(
        "api_skip_invalid_tool_call".to_string(),
        "Skipping invalid tool call:".to_string(),
    );
    m.insert(
        "api_skip_invalid_json_args".to_string(),
        "Skipping tool call with invalid JSON arguments:".to_string(),
    );
    m.insert(
        "api_tool_execution_error".to_string(),
        "Tool execution error: {}".to_string(),
    );
    m.insert(
        "api_skip_empty_tool_call".to_string(),
        "Skipping empty tool call:".to_string(),
    );
    m.insert(
        "api_incomplete_json".to_string(),
        "Incomplete JSON for tool".to_string(),
    );
    m.insert(
        "api_auto_fixed_json".to_string(),
        "Auto-fixed JSON for tool".to_string(),
    );
    m.insert(
        "api_failed_fix_json".to_string(),
        "Failed to fix JSON for tool".to_string(),
    );

    // Indexing
    m.insert("index_suggest_title".to_string(), "Suggest: Your codebase has changed significantly ({} commits) since the last index.".to_string());
    m.insert("index_suggest_action".to_string(), "Run '/index outline' to update the code outline for better search results.".to_string());
    m.insert("index_tip_title".to_string(), "Tip: Run '/index outline all' to build a code outline index for faster searches.".to_string());
    m.insert("index_usage_header".to_string(), "Usage: /index <subcommand>".to_string());
    m.insert("index_usage_outline".to_string(), "  outline       - Incrementally index the project outline".to_string());
    m.insert("index_usage_outline_all".to_string(), "  outline all   - Fully re-index the project outline".to_string());
    m.insert("index_start_full".to_string(), "🔍 Starting Full Code Outline Indexing...".to_string());
    m.insert("index_start_incremental".to_string(), "🔍 Starting Incremental Code Outline Indexing...".to_string());
    m.insert("index_no_files".to_string(), "⚠️  No supported source files found.".to_string());
    m.insert("index_found_files".to_string(), "📝 Found {} supported files.".to_string());
    m.insert("index_complete".to_string(), "✨ Indexing completed in {:.2?}".to_string());
    m.insert("index_stat_processed".to_string(), "   - Processed: {}".to_string());
    m.insert("index_stat_indexed".to_string(), "   - Indexed:   {}".to_string());
    m.insert("index_stat_failed".to_string(), "   - Failed:    {}".to_string());
    m.insert("index_unknown_subcommand".to_string(), "Unknown subcommand: {}".to_string());

    // File Replace Diagnostics
    m.insert("replace_diag_not_found".to_string(), "String to replace not found. Diagnostics:".to_string());
    m.insert("replace_diag_edit_num".to_string(), "Edit #{}:".to_string());
    m.insert("replace_diag_len".to_string(), "  Search string length: {} chars".to_string());
    m.insert("replace_diag_preview".to_string(), "  Search string (first 100 chars): {}".to_string());
    m.insert("replace_diag_has_newline".to_string(), "  Contains newline: {}".to_string());
    m.insert("replace_diag_has_crlf".to_string(), "  Contains \\r\\n: {}".to_string());
    m.insert("replace_diag_similar".to_string(), "  Found similar content in file (possible whitespace/newline mismatch):".to_string());
    m.insert("replace_diag_hints".to_string(), "Hints: Check for:\n  1. Line ending differences (Windows \\r\\n vs Unix \\n)\n  2. Extra whitespace\n  3. Tab/Space indentation differences\n  4. Special character encoding".to_string());
    m.insert("file_replace_not_found".to_string(), "Search string not found in file".to_string());
    m.insert("file_replace_success".to_string(), "Applied {} edits, total {} replacements in {1}".to_string());

    m.insert(
        "cmd_model_interactive".to_string(),
        "Interactive model selector".to_string(),
    );

    // Notification messages
    m.insert(
        "notify_ai_completed_body".to_string(),
        "Output completed, please check back.".to_string(),
    );

    // MCP System Messages
    m.insert("mcp_servers".to_string(), "MCP Servers".to_string());
    m.insert("mcp_integration_initialized".to_string(), "MCP integration initialized".to_string());
    m.insert("mcp_integration_failed".to_string(), "MCP integration failed".to_string());
    m.insert("mcp_not_available".to_string(), "MCP integration not available".to_string());
    m.insert("mcp_no_servers".to_string(), "No MCP servers configured".to_string());
    m.insert("mcp_server_loading".to_string(), "loading...".to_string());
    m.insert("mcp_connected".to_string(), "Connected".to_string());
    m.insert("mcp_failed_connect".to_string(), "Failed to connect".to_string());

    // MCP Commands
    m.insert("mcp_status".to_string(), "Show MCP server status".to_string());
    m.insert("mcp_tools".to_string(), "List all available tools".to_string());
    m.insert("mcp_tools_server".to_string(), "List tools for specific server".to_string());
    m.insert("mcp_resources".to_string(), "List all available resources".to_string());
    m.insert("mcp_resources_server".to_string(), "List resources for specific server".to_string());
    m.insert("mcp_call_tool".to_string(), "Call a tool".to_string());
    m.insert("mcp_read_resource".to_string(), "Read a resource".to_string());
    m.insert("mcp_help".to_string(), "Show MCP help".to_string());

    // MCP Status Messages
    m.insert("mcp_available_tools".to_string(), "Available MCP Tools".to_string());
    m.insert("mcp_no_tools".to_string(), "No tools available".to_string());
    m.insert("mcp_tools_for_server".to_string(), "Tools for server".to_string());
    m.insert("mcp_server_not_found".to_string(), "Server not found".to_string());
    m.insert("mcp_failed_get_tools".to_string(), "Failed to get tools".to_string());
    m.insert("mcp_calling_tool".to_string(), "Calling tool".to_string());
    m.insert("mcp_tool_result".to_string(), "Result".to_string());
    m.insert("mcp_tool_failed".to_string(), "Tool call failed".to_string());
    m.insert("mcp_reading_resource".to_string(), "Reading resource".to_string());
    m.insert("mcp_resource_content".to_string(), "Resource content".to_string());
    m.insert("mcp_resource_failed".to_string(), "Failed to read resource".to_string());

    // MCP Prompt System  
    m.insert("prompt_interactive_flow".to_string(), "Interactive Prompt Flow".to_string());
    m.insert("prompt_available_servers".to_string(), "Available MCP servers".to_string());
    m.insert("prompt_select_server".to_string(), "Select server ({}-{}) or 'q' to quit".to_string());
    m.insert("prompt_selected_server".to_string(), "Selected".to_string());
    m.insert("prompt_using_server".to_string(), "Using server".to_string());
    m.insert("prompt_getting_prompts".to_string(), "Getting prompts from".to_string());
    m.insert("prompt_failed_get_prompts".to_string(), "Failed to get prompts".to_string());
    m.insert("prompt_no_prompts".to_string(), "No prompts available from server".to_string());
    m.insert("prompt_using_prompt".to_string(), "Using prompt".to_string());
    m.insert("prompt_available_prompts".to_string(), "Available prompts".to_string());
    m.insert("prompt_select_prompt".to_string(), "Select prompt ({}-{}) or 'q' to quit".to_string());
    m.insert("prompt_selected_prompt".to_string(), "Selected".to_string());
    m.insert("prompt_executing".to_string(), "Executing prompt".to_string());
    m.insert("prompt_collecting_args".to_string(), "Collecting arguments for".to_string());
    m.insert("prompt_no_args_required".to_string(), "No arguments required".to_string());
    m.insert("prompt_arg_required".to_string(), "This argument is required".to_string());
    m.insert("prompt_result".to_string(), "Prompt Result".to_string());
    m.insert("prompt_completed".to_string(), "Prompt execution completed".to_string());
    m.insert("prompt_invalid_choice".to_string(), "Invalid choice. Please enter".to_string());

    // MCP Help Messages
    m.insert("mcp_commands_help".to_string(), "MCP Commands".to_string());
    m.insert("mcp_examples".to_string(), "Examples".to_string());
    m.insert("prompt_command_help".to_string(), "Prompt Command Help".to_string());
    m.insert("prompt_flow".to_string(), "Flow".to_string());
    m.insert("prompt_features".to_string(), "Features".to_string());
    m.insert("prompt_note".to_string(), "Note".to_string());
    m.insert("prompt_mcp_compliant".to_string(), "All prompts and their arguments are defined by the MCP servers.".to_string());
    m.insert("prompt_no_hardcoded".to_string(), "No hardcoded prompt types - everything is discovered dynamically.".to_string());

    // MCP Error Messages  
    m.insert("mcp_tool_error".to_string(), "MCP tool error".to_string());
    m.insert("mcp_resource_error".to_string(), "MCP resource error".to_string());
    m.insert("mcp_invalid_uri_format".to_string(), "Invalid MCP URI format. Use: mcp://server/resource".to_string());
    m.insert("mcp_tool_executed".to_string(), "MCP tool executed".to_string());
    m.insert("mcp_unknown_command".to_string(), "Unknown MCP command".to_string());

    // MCP Command Usage Messages
    m.insert("mcp_usage_call".to_string(), "Usage: mcp call <server> <tool> [args_json]".to_string());
    m.insert("mcp_usage_read".to_string(), "Usage: mcp read <server> <uri>".to_string());
    m.insert("mcp_available_resources".to_string(), "Available MCP Resources".to_string());
    m.insert("mcp_resources_for_server".to_string(), "Resources for server".to_string());
    m.insert("mcp_resource_not_implemented".to_string(), "Resource listing not implemented yet".to_string());
    m.insert("mcp_calling_tool_msg".to_string(), "Calling tool '{}' on server '{}'...".to_string());
    m.insert("mcp_reading_resource_msg".to_string(), "Reading resource '{}' from server '{}'...".to_string());

    // MCP Interactive Messages  
    m.insert("mcp_no_servers_connected".to_string(), "No MCP servers connected".to_string());
    m.insert("mcp_getting_prompts_from".to_string(), "Getting prompts from '{}'...".to_string());
    m.insert("mcp_invalid_choice_range".to_string(), "Invalid choice. Please enter 1-{} or 'q'".to_string());
    m.insert("mcp_collecting_args_for".to_string(), "Collecting arguments for '{}'".to_string());
    m.insert("mcp_prompt_result_header".to_string(), "Prompt Result".to_string());
    m.insert("mcp_prompt_execution_completed".to_string(), "Prompt execution completed".to_string());
    m.insert("mcp_sending_to_ai".to_string(), "Sending to AI for processing...".to_string());
    m.insert("mcp_ai_response_header".to_string(), "AI Response".to_string());
    m.insert("mcp_ai_response_failed".to_string(), "Failed to get AI response".to_string());
    m.insert("mcp_ai_tool_calls".to_string(), "AI wants to execute".to_string());
    
    // Prompt selection i18n
    m.insert("prompt_available_prompts".to_string(), "Available prompts".to_string());
    m.insert("prompt_select_or_quit".to_string(), "Select prompt (1-{}) or 'q' to quit".to_string());
    m.insert("prompt_selected".to_string(), "Selected".to_string());
    m.insert("prompt_executing".to_string(), "Executing prompt".to_string());
    m.insert("prompt_collecting_args".to_string(), "Collecting arguments for".to_string());
    m.insert("prompt_result_header".to_string(), "Prompt Result".to_string());

    m
}

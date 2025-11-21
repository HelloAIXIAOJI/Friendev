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

    // Tool messages
    m.insert("tool_call".to_string(), "TOOL".to_string());
    m.insert("thinking".to_string(), "THINK".to_string());

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

    // UI: approval prompt & details
    m.insert(
        "approval_title".to_string(),
        "  ──── Approval Required ──────────────────".to_string(),
    );
    m.insert(
        "approval_empty_line".to_string(),
        "                                           ".to_string(),
    );
    m.insert(
        "approval_separator".to_string(),
        "  ─────────────────────────────────────────".to_string(),
    );
    m.insert(
        "approval_action_wants".to_string(),
        "wants to modify".to_string(),
    );
    m.insert(
        "approval_content_preview".to_string(),
        "Content preview:".to_string(),
    );
    m.insert(
        "approval_choice_hint".to_string(),
        "[Y]es / [N]o / [I]nfo / [A]lways".to_string(),
    );
    m.insert(
        "approval_choice_prompt".to_string(),
        "Your choice:".to_string(),
    );
    m.insert(
        "approval_always_approved".to_string(),
        "Approved for this session".to_string(),
    );
    m.insert("approval_rejected".to_string(), "Rejected".to_string());

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

    m.insert(
        "approval_user_rejected".to_string(),
        "User rejected this operation".to_string(),
    );
    m.insert(
        "approval_user_cancelled".to_string(),
        "User cancelled this operation".to_string(),
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

    m
}

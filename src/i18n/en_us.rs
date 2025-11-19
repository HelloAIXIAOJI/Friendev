use std::collections::HashMap;

pub fn get_messages() -> HashMap<String, String> {
    let mut m = HashMap::new();
    
    // Startup messages
    m.insert("config_loaded".to_string(), "Config loaded".to_string());
    m.insert("working_dir".to_string(), "Working directory".to_string());
    m.insert("new_session".to_string(), "New session".to_string());
    m.insert("welcome_subtitle".to_string(), "AI-Powered Development Assistant".to_string());
    m.insert("current_model".to_string(), "Current Model".to_string());
    m.insert("available_commands".to_string(), "Available Commands".to_string());
    m.insert("type_message".to_string(), ">> Type a message to start chatting".to_string());
    
    // Commands
    m.insert("cmd_model_list".to_string(), "List all models".to_string());
    m.insert("cmd_model_switch".to_string(), "Switch model".to_string());
    m.insert("cmd_history_list".to_string(), "List chat history".to_string());
    m.insert("cmd_history_switch".to_string(), "Switch session".to_string());
    m.insert("cmd_history_new".to_string(), "Create new session".to_string());
    m.insert("cmd_history_del".to_string(), "Delete session".to_string());
    m.insert("cmd_language_ui".to_string(), "Set UI language".to_string());
    m.insert("cmd_language_ai".to_string(), "Set AI language".to_string());
    m.insert("cmd_help".to_string(), "Show help".to_string());
    m.insert("cmd_exit".to_string(), "Exit program".to_string());
    m.insert("cmd_agents_md".to_string(), "Generate AGENTS.md file".to_string());
    m.insert("cmd_runcommand_list".to_string(), "List commands requiring approval".to_string());
    m.insert("cmd_runcommand_add".to_string(), "Add command to approval list".to_string());
    m.insert("cmd_runcommand_del".to_string(), "Remove command from approval list".to_string());
    m.insert("cmd_runcommand_info".to_string(), "Show background command details".to_string());
    
    // Status messages
    m.insert("goodbye".to_string(), "Goodbye!".to_string());
    m.insert("loading_models".to_string(), "Loading models...".to_string());
    m.insert("available_models".to_string(), "Available Models".to_string());
    m.insert("switched_model".to_string(), "Switched to model".to_string());
    m.insert("switched_session".to_string(), "Switched to session".to_string());
    m.insert("created_session".to_string(), "Created new session".to_string());
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
    m.insert("failed_load_models".to_string(), "Failed to load models".to_string());
    m.insert("failed_load_session".to_string(), "Failed to load session".to_string());
    m.insert("invalid_uuid".to_string(), "Invalid UUID".to_string());
    m.insert("cannot_delete_current".to_string(), "Cannot delete current session".to_string());
    
    // Help
    m.insert("help_title".to_string(), "Help - Available Commands".to_string());
    m.insert("help_model".to_string(), "Model Commands".to_string());
    m.insert("help_history".to_string(), "History Commands".to_string());
    m.insert("help_language".to_string(), "Language Commands".to_string());
    m.insert("help_other".to_string(), "Other Commands".to_string());
    m.insert("help_runcommand".to_string(), "Run Command Settings".to_string());
    
    // Language
    m.insert("ui_language_set".to_string(), "UI language set to".to_string());
    m.insert("ai_language_set".to_string(), "AI response language set to".to_string());
    m.insert("supported_languages".to_string(), "AI support depends on the model.".to_string());
    m.insert("current_ui_lang".to_string(), "Current UI Language".to_string());
    m.insert("current_ai_lang".to_string(), "Current AI Language".to_string());
    
    // Setup initialization
    m.insert("setup_welcome".to_string(), "Welcome to Friendev! First-time use requires initialization configuration.".to_string());
    m.insert("setup_api_key".to_string(), "Please enter OpenAI API Key".to_string());
    m.insert("setup_api_url".to_string(), "Please enter OpenAI Base URL".to_string());
    m.insert("setup_model".to_string(), "Please enter the default model".to_string());
    m.insert("setup_ui_language".to_string(), "Please select UI language".to_string());
    m.insert("setup_ai_language".to_string(), "Please enter AI response language".to_string());
    m.insert("setup_saved".to_string(), "Configuration saved!".to_string());

    // UI: approval prompt & details
    m.insert("approval_title".to_string(), "  ──── Approval Required ──────────────────".to_string());
    m.insert("approval_empty_line".to_string(), "                                           ".to_string());
    m.insert("approval_separator".to_string(), "  ─────────────────────────────────────────".to_string());
    m.insert("approval_action_wants".to_string(), "wants to modify".to_string());
    m.insert("approval_content_preview".to_string(), "Content preview:".to_string());
    m.insert("approval_choice_hint".to_string(), "[Y]es / [N]o / [I]nfo / [A]lways".to_string());
    m.insert("approval_choice_prompt".to_string(), "Your choice:".to_string());
    m.insert("approval_always_approved".to_string(), "Approved for this session".to_string());
    m.insert("approval_rejected".to_string(), "Rejected".to_string());

    m.insert("details_title".to_string(), "  ──── Detailed Code Changes ──────────────────".to_string());
    m.insert("details_separator".to_string(), "  ──────────────────────────────────────────".to_string());
    m.insert("details_tool".to_string(), "Tool:".to_string());
    m.insert("details_file".to_string(), "File:".to_string());
    m.insert("details_choice_hint".to_string(), "[C]ontinue / [A]bort".to_string());
    m.insert("details_choice_prompt".to_string(), "Your choice:".to_string());

    // UI: tool call display
    m.insert("tool_action_used".to_string(), "Used".to_string());
    m.insert("tool_action_using".to_string(), "Using".to_string());
    
    m
}

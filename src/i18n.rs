use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct I18n {
    language: String,
    messages: HashMap<String, String>,
}

impl I18n {
    pub fn new(language: &str) -> Self {
        let messages = match language {
            "zh" | "zh-CN" => Self::zh_cn(),
            _ => Self::en_us(),
        };
        
        Self {
            language: language.to_string(),
            messages,
        }
    }

    pub fn get(&self, key: &str) -> String {
        self.messages
            .get(key)
            .cloned()
            .unwrap_or_else(|| format!("[Missing: {}]", key))
    }

    fn en_us() -> HashMap<String, String> {
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
        
        // Language
        m.insert("ui_language_set".to_string(), "UI language set to".to_string());
        m.insert("ai_language_set".to_string(), "AI response language set to".to_string());
        m.insert("supported_languages".to_string(), "UI support: en and zh. AI support: depending on the model.".to_string());
        m.insert("current_ui_lang".to_string(), "Current UI Language".to_string());
        m.insert("current_ai_lang".to_string(), "Current AI Language".to_string());
        
        m
    }

    fn zh_cn() -> HashMap<String, String> {
        let mut m = HashMap::new();
        
        // 启动消息
        m.insert("config_loaded".to_string(), "配置已加载".to_string());
        m.insert("working_dir".to_string(), "工作目录".to_string());
        m.insert("new_session".to_string(), "新会话".to_string());
        m.insert("welcome_subtitle".to_string(), "AI 驱动的开发助手".to_string());
        m.insert("current_model".to_string(), "当前模型".to_string());
        m.insert("available_commands".to_string(), "可用命令".to_string());
        m.insert("type_message".to_string(), ">> 输入消息开始对话".to_string());
        
        // 命令
        m.insert("cmd_model_list".to_string(), "列出所有模型".to_string());
        m.insert("cmd_model_switch".to_string(), "切换模型".to_string());
        m.insert("cmd_history_list".to_string(), "列出聊天历史".to_string());
        m.insert("cmd_history_switch".to_string(), "切换会话".to_string());
        m.insert("cmd_history_new".to_string(), "创建新会话".to_string());
        m.insert("cmd_history_del".to_string(), "删除会话".to_string());
        m.insert("cmd_language_ui".to_string(), "设置界面语言".to_string());
        m.insert("cmd_language_ai".to_string(), "设置 AI 语言".to_string());
        m.insert("cmd_help".to_string(), "显示帮助".to_string());
        m.insert("cmd_exit".to_string(), "退出程序".to_string());
        
        // 状态消息
        m.insert("goodbye".to_string(), "再见！".to_string());
        m.insert("loading_models".to_string(), "正在加载模型...".to_string());
        m.insert("available_models".to_string(), "可用模型".to_string());
        m.insert("switched_model".to_string(), "已切换到模型".to_string());
        m.insert("switched_session".to_string(), "已切换到会话".to_string());
        m.insert("created_session".to_string(), "已创建新会话".to_string());
        m.insert("deleted_session".to_string(), "已删除会话".to_string());
        m.insert("no_history".to_string(), "没有聊天历史".to_string());
        m.insert("chat_history".to_string(), "聊天历史".to_string());
        m.insert("messages".to_string(), "条消息".to_string());
        
        // 工具消息
        m.insert("tool_call".to_string(), "工具".to_string());
        m.insert("thinking".to_string(), "思考".to_string());
        
        // 错误消息
        m.insert("error".to_string(), "错误".to_string());
        m.insert("api_error".to_string(), "API 错误".to_string());
        m.insert("unknown_command".to_string(), "未知命令".to_string());
        m.insert("usage".to_string(), "用法".to_string());
        m.insert("failed_load_models".to_string(), "加载模型失败".to_string());
        m.insert("failed_load_session".to_string(), "加载会话失败".to_string());
        m.insert("invalid_uuid".to_string(), "无效的 UUID".to_string());
        m.insert("cannot_delete_current".to_string(), "无法删除当前会话".to_string());
        
        // 帮助
        m.insert("help_title".to_string(), "帮助 - 可用命令".to_string());
        m.insert("help_model".to_string(), "模型命令".to_string());
        m.insert("help_history".to_string(), "历史命令".to_string());
        m.insert("help_language".to_string(), "语言命令".to_string());
        m.insert("help_other".to_string(), "其他命令".to_string());
        
        // 语言
        m.insert("ui_language_set".to_string(), "界面语言已设置为".to_string());
        m.insert("ai_language_set".to_string(), "AI 回复语言已设置为".to_string());
        m.insert("supported_languages".to_string(), "UI支持: en与zh。AI支持：根据模型而定。".to_string());
        m.insert("current_ui_lang".to_string(), "当前界面语言".to_string());
        m.insert("current_ai_lang".to_string(), "当前 AI 语言".to_string());
        
        m
    }
}

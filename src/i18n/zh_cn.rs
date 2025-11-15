use std::collections::HashMap;

pub fn get_messages() -> HashMap<String, String> {
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
    m.insert("cmd_agents_md".to_string(), "生成 AGENTS.md 文件".to_string());
    m.insert("cmd_runcommand_list".to_string(), "列出需要审批的命令".to_string());
    m.insert("cmd_runcommand_add".to_string(), "添加命令到审批列表".to_string());
    m.insert("cmd_runcommand_del".to_string(), "从审批列表移除命令".to_string());
    m.insert("cmd_runcommand_info".to_string(), "显示后台命令详情".to_string());
    
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
    m.insert("help_runcommand".to_string(), "运行命令设置".to_string());
    
    // 语言
    m.insert("ui_language_set".to_string(), "界面语言已设置为".to_string());
    m.insert("ai_language_set".to_string(), "AI 回复语言已设置为".to_string());
    m.insert("supported_languages".to_string(), "AI 支持取决于所用模型。".to_string());
    m.insert("current_ui_lang".to_string(), "当前界面语言".to_string());
    m.insert("current_ai_lang".to_string(), "当前 AI 语言".to_string());
    
    // 初始化设置
    m.insert("setup_welcome".to_string(), "欢迎使用 Friendev！首次使用需要初始化配置。".to_string());
    m.insert("setup_api_key".to_string(), "请输入 OpenAI API Key".to_string());
    m.insert("setup_api_url".to_string(), "请输入 OpenAI API URL".to_string());
    m.insert("setup_model".to_string(), "请输入默认模型".to_string());
    m.insert("setup_ui_language".to_string(), "请选择界面语言".to_string());
    m.insert("setup_ai_language".to_string(), "请输入 AI 回复语言".to_string());
    m.insert("setup_saved".to_string(), "配置已保存！".to_string());
    
    m
}

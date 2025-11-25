use std::collections::HashMap;

pub fn get_messages() -> HashMap<String, String> {
    let mut m = HashMap::new();

    // 启动消息
    m.insert("config_loaded".to_string(), "配置已加载".to_string());
    m.insert("working_dir".to_string(), "工作目录".to_string());
    m.insert("new_session".to_string(), "新会话".to_string());
    m.insert(
        "welcome_subtitle".to_string(),
        "AI 驱动的开发助手".to_string(),
    );
    m.insert("current_model".to_string(), "当前模型".to_string());
    m.insert("available_commands".to_string(), "可用命令".to_string());
    m.insert(
        "type_message".to_string(),
        ">> 输入消息开始对话".to_string(),
    );

    // 命令
    m.insert(
        "cmd_model_interactive".to_string(),
        "交互式模型选择器".to_string(),
    );
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
    m.insert(
        "cmd_agents_md".to_string(),
        "生成 AGENTS.md 文件".to_string(),
    );
    m.insert(
        "cmd_runcommand_list".to_string(),
        "列出需要审批的命令".to_string(),
    );
    m.insert(
        "cmd_runcommand_add".to_string(),
        "添加命令到审批列表".to_string(),
    );
    m.insert(
        "cmd_runcommand_del".to_string(),
        "从审批列表移除命令".to_string(),
    );
    m.insert(
        "cmd_runcommand_info".to_string(),
        "显示后台命令详情".to_string(),
    );

    // 状态消息
    m.insert("goodbye".to_string(), "再见！".to_string());
    m.insert("loading_models".to_string(), "正在加载模型...".to_string());
    m.insert("available_models".to_string(), "可用模型".to_string());
    m.insert("switched_model".to_string(), "已切换到模型".to_string());
    m.insert(
        "model_selector_prompt".to_string(),
        "选择一个模型".to_string(),
    );
    m.insert(
        "model_already_active".to_string(),
        "模型已经是当前活动模型:".to_string(),
    );
    m.insert(
        "model_selection_cancelled".to_string(),
        "已取消模型选择".to_string(),
    );
    m.insert(
        "no_models_available".to_string(),
        "没有可用的模型".to_string(),
    );
    m.insert(
        "interactive_mode_failed".to_string(),
        "交互模式不可用，显示列表".to_string(),
    );
    m.insert("switched_session".to_string(), "已切换到会话".to_string());
    m.insert("created_session".to_string(), "已创建新会话".to_string());
    m.insert("deleted_session".to_string(), "已删除会话".to_string());
    m.insert("no_history".to_string(), "没有聊天历史".to_string());
    m.insert("chat_history".to_string(), "聊天历史".to_string());
    m.insert("messages".to_string(), "条消息".to_string());

    // 工具消息
    m.insert("tool_call".to_string(), "工具".to_string());
    m.insert("thinking".to_string(), "思考".to_string());
    m.insert("tools_header".to_string(), "使用工具".to_string());

    // 错误消息
    m.insert("error".to_string(), "错误".to_string());
    m.insert("api_error".to_string(), "API 错误".to_string());
    m.insert("unknown_command".to_string(), "未知命令".to_string());
    m.insert("usage".to_string(), "用法".to_string());
    m.insert("failed_load_models".to_string(), "加载模型失败".to_string());
    m.insert(
        "failed_load_session".to_string(),
        "加载会话失败".to_string(),
    );
    m.insert("invalid_uuid".to_string(), "无效的 UUID".to_string());
    m.insert(
        "cannot_delete_current".to_string(),
        "无法删除当前会话".to_string(),
    );

    // 帮助
    m.insert("help_title".to_string(), "帮助 - 可用命令".to_string());
    m.insert("help_model".to_string(), "模型命令".to_string());
    m.insert("help_history".to_string(), "历史命令".to_string());
    m.insert("help_language".to_string(), "语言命令".to_string());
    m.insert("help_other".to_string(), "其他命令".to_string());
    m.insert("help_runcommand".to_string(), "运行命令设置".to_string());

    // 语言
    m.insert(
        "ui_language_set".to_string(),
        "界面语言已设置为".to_string(),
    );
    m.insert(
        "ai_language_set".to_string(),
        "AI 回复语言已设置为".to_string(),
    );
    m.insert(
        "supported_languages".to_string(),
        "AI 支持取决于所用模型。".to_string(),
    );
    m.insert("current_ui_lang".to_string(), "当前界面语言".to_string());
    m.insert("current_ai_lang".to_string(), "当前 AI 语言".to_string());

    // 初始化设置
    m.insert(
        "setup_welcome".to_string(),
        "欢迎使用 Friendev！首次使用需要初始化配置。".to_string(),
    );
    m.insert(
        "setup_api_key".to_string(),
        "请输入 OpenAI API Key".to_string(),
    );
    m.insert(
        "setup_api_url".to_string(),
        "请输入 OpenAI API URL".to_string(),
    );
    m.insert("setup_model".to_string(), "请输入默认模型".to_string());
    m.insert(
        "setup_ui_language".to_string(),
        "请选择界面语言".to_string(),
    );
    m.insert(
        "setup_ai_language".to_string(),
        "请输入 AI 回复语言".to_string(),
    );
    m.insert("setup_saved".to_string(), "配置已保存！".to_string());

    // UI：审批提示与详情（已不再在运行时使用，仅保留审查相关文案）
    m.insert(
        "approval_review_unavailable".to_string(),
        "当前无法使用审查助手".to_string(),
    );
    m.insert(
        "approval_review_error".to_string(),
        "审查失败：{}".to_string(),
    );
    m.insert(
        "approval_review_request".to_string(),
        "正在请求 AI 审查操作 '{}'。".to_string(),
    );
    m.insert(
        "approval_review_wait".to_string(),
        "等待审查返回结果...".to_string(),
    );
    m.insert("approval_review_done".to_string(), "审查完成".to_string());
    m.insert(
        "approval_review_result".to_string(),
        "AI 审查结果：".to_string(),
    );
    m.insert(
        "approval_review_tool_error".to_string(),
        "审查返回了当前不支持的工具调用".to_string(),
    );
    m.insert(
        "approval_review_no_preview".to_string(),
        "（无更多预览信息）".to_string(),
    );
    m.insert(
        "approval_review_parse_error".to_string(),
        "无法解析审查结果：{}".to_string(),
    );
    m.insert("approval_review_raw".to_string(), "原始响应:".to_string());
    m.insert("approval_review_decision".to_string(), "建议:".to_string());
    m.insert("approval_review_details".to_string(), "详情:".to_string());
    m.insert(
        "approval_review_followup".to_string(),
        "审查完成，请输入最终决定（仅限 Y/N）。".to_string(),
    );
    m.insert(
        "approval_review_decision_prompt".to_string(),
        "最终决定 [Y/N]:".to_string(),
    );
    m.insert(
        "approval_review_invalid_choice".to_string(),
        "输入无效，请输入 Y 或 N。".to_string(),
    );
    m.insert(
        "approval_review_decision_yes".to_string(),
        "同意执行".to_string(),
    );
    m.insert(
        "approval_review_decision_no".to_string(),
        "拒绝执行".to_string(),
    );

    m.insert(
        "details_title".to_string(),
        "  ──── 代码变更详情 ──────────────────".to_string(),
    );
    m.insert(
        "details_separator".to_string(),
        "  ──────────────────────────────────────────".to_string(),
    );
    m.insert("details_tool".to_string(), "工具:".to_string());
    m.insert("details_file".to_string(), "文件:".to_string());
    m.insert(
        "details_choice_hint".to_string(),
        "[C]继续 / [A]终止".to_string(),
    );
    m.insert(
        "details_choice_prompt".to_string(),
        "请输入选择:".to_string(),
    );

    // UI：工具调用展示
    m.insert("tool_action_used".to_string(), "已使用".to_string());
    m.insert("tool_action_using".to_string(), "正在使用".to_string());

    // Tools & executor messages
    m.insert("tool_unknown".to_string(), "未知工具: {}".to_string());

    m.insert("file_not_exist".to_string(), "文件不存在: {}".to_string());
    m.insert("file_not_file".to_string(), "不是文件: {}".to_string());
    m.insert(
        "file_path_not_exist".to_string(),
        "路径不存在: {}".to_string(),
    );
    m.insert("file_not_directory".to_string(), "不是目录: {}".to_string());

    m.insert("file_item_type_dir".to_string(), "目录".to_string());
    m.insert("file_item_type_file".to_string(), "文件".to_string());
    m.insert("file_item_size_na".to_string(), "-".to_string());
    m.insert("file_list_item".to_string(), "{} [{}] ({})".to_string());
    m.insert("file_list_empty".to_string(), "目录为空".to_string());
    m.insert("file_list_brief".to_string(), "列出 {} 项".to_string());
    m.insert("file_list_header".to_string(), "目录: {}".to_string());
    m.insert("file_list_count".to_string(), "共 {} 项:".to_string());

    m.insert(
        "file_read_brief".to_string(),
        "读取 {} 行, {} 字节".to_string(),
    );
    m.insert(
        "file_read_header".to_string(),
        "文件: {}\n内容:".to_string(),
    );

    m.insert(
        "file_write_invalid_mode".to_string(),
        "无效的写入模式: {}，只支持 'overwrite' 或 'append'".to_string(),
    );
    m.insert(
        "file_write_append_action".to_string(),
        "追加到文件: {}".to_string(),
    );
    m.insert(
        "file_write_overwrite_action".to_string(),
        "覆盖文件: {}".to_string(),
    );
    m.insert(
        "file_write_append_brief".to_string(),
        "追加 {} 字节".to_string(),
    );
    m.insert(
        "file_write_append_output".to_string(),
        "成功追加到文件: {}\n追加: {} 字节\n当前大小: {} 字节".to_string(),
    );
    m.insert(
        "file_write_overwrite_brief".to_string(),
        "写入 {} 字节".to_string(),
    );
    m.insert(
        "file_write_overwrite_output".to_string(),
        "成功写入文件: {}\n大小: {} 字节".to_string(),
    );

    // Search tool messages
    m.insert(
        "search_engine_prefix".to_string(),
        "搜索引擎: {}\n".to_string(),
    );
    m.insert("search_keywords_label".to_string(), "关键词".to_string());
    m.insert("search_found_label".to_string(), "找到".to_string());
    m.insert("search_url_label".to_string(), "URL".to_string());
    m.insert("search_snippet_label".to_string(), "摘要".to_string());
    m.insert(
        "search_brief_with_engine".to_string(),
        "{}: 找到 {} 个结果".to_string(),
    );
    m.insert("search_brief".to_string(), "找到 {} 个结果".to_string());
    m.insert(
        "search_error_with_engine".to_string(),
        "{}搜索失败: {}".to_string(),
    );
    m.insert("search_error".to_string(), "搜索失败: {}".to_string());
    m.insert(
        "search_ddg_no_results".to_string(),
        "DuckDuckGo 未找到搜索结果".to_string(),
    );
    m.insert(
        "search_bing_request_failed".to_string(),
        "Bing 请求失败".to_string(),
    );
    m.insert(
        "search_bing_status_code".to_string(),
        "Bing 返回状态码".to_string(),
    );
    m.insert(
        "search_bing_read_failed".to_string(),
        "读取 Bing 响应失败".to_string(),
    );
    m.insert(
        "search_bing_no_results".to_string(),
        "Bing 未找到搜索结果".to_string(),
    );
    m.insert(
        "search_ddg_error_prefix".to_string(),
        "DuckDuckGo 错误".to_string(),
    );
    m.insert(
        "search_try_bing".to_string(),
        "尝试使用 Bing...".to_string(),
    );

    // Network fetch tool messages
    m.insert(
        "network_fetch_invalid_url".to_string(),
        "无效的 URL：{}".to_string(),
    );
    m.insert(
        "network_fetch_unsupported_scheme".to_string(),
        "不支持的 URL 协议：{}（仅允许 http 或 https）".to_string(),
    );
    m.insert(
        "network_fetch_request_error".to_string(),
        "请求 URL 失败：{}".to_string(),
    );
    m.insert(
        "network_fetch_timeout".to_string(),
        "请求 URL 超时。".to_string(),
    );
    m.insert(
        "network_fetch_status_error".to_string(),
        "请求失败，状态码 {}（{}）".to_string(),
    );
    m.insert(
        "network_fetch_too_large".to_string(),
        "响应体过大（限制 {}）。".to_string(),
    );
    m.insert(
        "network_fetch_non_text".to_string(),
        "不支持的内容类型：{}（仅允许文本内容）。".to_string(),
    );
    m.insert(
        "network_fetch_brief".to_string(),
        "成功获取 {} 数据。".to_string(),
    );
    m.insert(
        "network_fetch_brief_truncated".to_string(),
        "成功获取 {} 数据（已截断）。".to_string(),
    );
    m.insert(
        "network_fetch_truncated_note".to_string(),
        "注意：内容已截断至 {}。".to_string(),
    );
    m.insert(
        "network_fetch_html_note".to_string(),
        "注意：HTML 内容已转换为纯文本。".to_string(),
    );
    m.insert(
        "network_fetch_output".to_string(),
        "URL：{}\n状态：{}\nContent-Type：{}\n大小：{}\n{}\n内容：\n{}".to_string(),
    );

    // Run command tool messages
    m.insert(
        "run_command_user_cancelled".to_string(),
        "用户取消了该操作".to_string(),
    );
    m.insert(
        "run_command_user_rejected".to_string(),
        "用户拒绝了该操作".to_string(),
    );
    m.insert(
        "run_command_bg_brief".to_string(),
        "已启动后台命令: {}".to_string(),
    );
    m.insert(
        "run_command_bg_output".to_string(),
        "命令已在后台启动\n运行 ID: {}\n命令: {}\n\n使用 /runcommand info {{}} 查看状态"
            .to_string(),
    );
    m.insert(
        "run_command_fg_brief".to_string(),
        "命令已执行: {} (退出码: {})".to_string(),
    );
    m.insert(
        "run_command_fg_output".to_string(),
        "命令: {}\n退出码: {}\n状态: {}\n\n输出:\n{}".to_string(),
    );
    m.insert(
        "run_command_execute_error".to_string(),
        "执行命令失败: {}".to_string(),
    );

    // Language command extras
    m.insert(
        "lang_ui_unsupported".to_string(),
        "不支持的界面语言: '{}'".to_string(),
    );
    m.insert("lang_supported_label".to_string(), "支持的语言".to_string());
    m.insert(
        "lang_supported_ui_label".to_string(),
        "支持的界面语言:".to_string(),
    );

    // Runcommand command messages
    m.insert(
        "runcommand_no_commands".to_string(),
        "当前没有需要审批的命令".to_string(),
    );
    m.insert(
        "runcommand_list_header".to_string(),
        "需要审批的命令".to_string(),
    );
    m.insert(
        "runcommand_load_config_failed".to_string(),
        "加载命令配置失败".to_string(),
    );
    m.insert(
        "runcommand_add_ok".to_string(),
        "已将 '{}' 添加到审批列表".to_string(),
    );
    m.insert(
        "runcommand_add_exists".to_string(),
        "'{}' 已在审批列表中".to_string(),
    );
    m.insert(
        "runcommand_del_ok".to_string(),
        "已从审批列表移除 '{}'".to_string(),
    );
    m.insert(
        "runcommand_del_not_found".to_string(),
        "'{}' 不在审批列表中".to_string(),
    );
    m.insert(
        "runcommand_info_header".to_string(),
        "后台命令信息".to_string(),
    );
    m.insert("runcommand_info_id".to_string(), "ID:".to_string());
    m.insert("runcommand_info_command".to_string(), "命令:".to_string());
    m.insert("runcommand_info_status".to_string(), "状态:".to_string());
    m.insert(
        "runcommand_info_started".to_string(),
        "开始时间:".to_string(),
    );
    m.insert(
        "runcommand_info_exit_code".to_string(),
        "退出码:".to_string(),
    );
    m.insert("runcommand_info_output".to_string(), "输出".to_string());
    m.insert(
        "runcommand_info_not_found".to_string(),
        "未找到 ID 为 '{}' 的命令".to_string(),
    );
    m.insert(
        "runcommand_help_header".to_string(),
        "/runcommand 帮助".to_string(),
    );

    // Agents command messages
    m.insert(
        "agents_analyzing_project".to_string(),
        "正在分析项目结构...".to_string(),
    );
    m.insert(
        "agents_sending_to_ai".to_string(),
        "正在发送给 AI 生成 AGENTS.md...".to_string(),
    );

    // History maintenance messages
    m.insert(
        "history_cleanup_empty".to_string(),
        "已清理 {} 个空会话".to_string(),
    );

    // History summary
    m.insert("history_new_chat_summary".to_string(), "新聊天".to_string());

    // Chat output labels
    m.insert("chat_think_label".to_string(), "思考".to_string());
    m.insert("chat_ai_label".to_string(), "AI".to_string());
    m.insert(
        "chat_tool_parse_error".to_string(),
        "检测到工具调用，但全部解析失败".to_string(),
    );
    m.insert("chat_debug_info_label".to_string(), "调试信息".to_string());
    m.insert(
        "chat_tool_parse_debug".to_string(),
        "请检查工具参数是否为合法 JSON".to_string(),
    );

    // Security messages
    m.insert("security_warning_label".to_string(), "安全警告".to_string());
    m.insert(
        "security_forbidden_tokens".to_string(),
        "输入包含禁止的控制标记".to_string(),
    );

    // API messages
    m.insert("api_retry_label".to_string(), "重试".to_string());
    m.insert("api_retry_waiting".to_string(), "等待".to_string());
    m.insert(
        "api_retries_failed".to_string(),
        "所有重试均已失败".to_string(),
    );
    m.insert("api_request_failed".to_string(), "请求失败".to_string());
    m.insert(
        "api_models_failed".to_string(),
        "获取模型列表失败".to_string(),
    );
    m.insert("api_stream_error".to_string(), "流错误: {}".to_string());
    m.insert(
        "api_skip_invalid_tool_call".to_string(),
        "跳过无效的工具调用:".to_string(),
    );
    m.insert(
        "api_skip_invalid_json_args".to_string(),
        "跳过 JSON 参数无效的工具调用:".to_string(),
    );
    m.insert(
        "api_tool_execution_error".to_string(),
        "工具执行错误: {}".to_string(),
    );
    m.insert(
        "api_skip_empty_tool_call".to_string(),
        "跳过空的工具调用:".to_string(),
    );
    m.insert(
        "api_incomplete_json".to_string(),
        "工具的 JSON 不完整".to_string(),
    );
    m.insert(
        "api_auto_fixed_json".to_string(),
        "已自动修复工具的 JSON".to_string(),
    );
    m.insert(
        "api_failed_fix_json".to_string(),
        "修复工具 JSON 失败".to_string(),
    );

    m
}

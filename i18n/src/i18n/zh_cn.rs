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

    // 提示消息
    m.insert("hint_send".to_string(), "发送".to_string());
    m.insert("hint_newline".to_string(), "换行".to_string());
    m.insert(
        "hint_short".to_string(),
        "Enter 发送  |  Alt+Enter 换行  |  Ctrl+Enter 换行".to_string(),
    );
    m.insert(
        "hint_shift_enter".to_string(),
        "Shift+Enter 或 ! 开头 = AI优化提示词".to_string(),
    );
    m.insert(
        "hint_esc".to_string(),
        "ESC 停止生成".to_string(),
    );
    m.insert(
        "hint_ctrl_c_twice".to_string(),
        "再按一次 Ctrl+C 退出".to_string(),
    );
    m.insert(
        "hint_ctrl_c_exit".to_string(),
        "Ctrl+C 两次退出".to_string(),
    );

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

    // Approval UI
    m.insert("approval_title".to_string(), "需要审批".to_string());
    m.insert("approval_action_wants".to_string(), "请求执行".to_string());
    m.insert("approval_content_preview".to_string(), "内容预览".to_string());
    m.insert("approval_always_approved".to_string(), "本会话总是批准".to_string());
    m.insert("approval_rejected".to_string(), "操作已拒绝".to_string());
    m.insert("approval_opt_approve".to_string(), "批准".to_string());
    m.insert("approval_opt_always".to_string(), "总是批准 (本会话)".to_string());
    m.insert("approval_opt_details".to_string(), "查看详情 / 审查".to_string());
    m.insert("approval_opt_reject".to_string(), "拒绝".to_string());

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

    // Indexing
    m.insert("index_suggest_title".to_string(), "建议：您的代码库自上次索引以来已有显著变更（{} 次提交）。".to_string());
    m.insert("index_suggest_action".to_string(), "运行 '/index outline' 更新代码大纲，以获得更佳的搜索结果。".to_string());
    m.insert("index_tip_title".to_string(), "提示：运行 '/index outline all' 构建代码大纲索引，加快搜索速度。".to_string());
    m.insert("index_usage_header".to_string(), "用法: /index <子命令>".to_string());
    m.insert("index_usage_outline".to_string(), "  outline       - 增量索引项目大纲".to_string());
    m.insert("index_usage_outline_all".to_string(), "  outline all   - 全量重建项目大纲索引".to_string());
    m.insert("index_start_full".to_string(), "🔍 开始全量代码大纲索引...".to_string());
    m.insert("index_start_incremental".to_string(), "🔍 开始增量代码大纲索引...".to_string());
    m.insert("index_no_files".to_string(), "⚠️  未找到支持的源文件。".to_string());
    m.insert("index_found_files".to_string(), "📝 找到 {} 个支持的文件。".to_string());
    m.insert("index_complete".to_string(), "✨ 索引完成，耗时 {:.2?}".to_string());
    m.insert("index_stat_processed".to_string(), "   - 已处理: {}".to_string());
    m.insert("index_stat_indexed".to_string(), "   - 已索引:   {}".to_string());
    m.insert("index_stat_failed".to_string(), "   - 失败:    {}".to_string());
    m.insert("index_unknown_subcommand".to_string(), "未知子命令: {}".to_string());

    // File Replace Diagnostics
    m.insert("replace_diag_not_found".to_string(), "未找到要替换的字符串。诊断信息：".to_string());
    m.insert("replace_diag_edit_num".to_string(), "编辑 #{}:".to_string());
    m.insert("replace_diag_len".to_string(), "  搜索字符串长度: {} 字符".to_string());
    m.insert("replace_diag_preview".to_string(), "  搜索字符串 (前100字符): {}".to_string());
    m.insert("replace_diag_has_newline".to_string(), "  包含换行符: {}".to_string());
    m.insert("replace_diag_has_crlf".to_string(), "  包含 \\r\\n: {}".to_string());
    m.insert("replace_diag_similar".to_string(), "  文件中发现相似内容（可能是空格/换行符差异）:".to_string());
    m.insert("replace_diag_hints".to_string(), "提示：检查以下可能的问题:\n  1. 行结束符差异 (Windows \\r\\n vs Unix \\n)\n  2. 前后有额外空格\n  3. 缩进使用了不同的制表符或空格\n  4. 特殊字符编码差异".to_string());
    m.insert("file_replace_not_found".to_string(), "文件中未找到搜索字符串".to_string());
    m.insert("file_replace_success".to_string(), "应用了 {} 个编辑，共 {} 次替换，文件：{1}".to_string());

    m.insert(
        "cmd_model_interactive".to_string(),
        "交互式模型选择器".to_string(),
    );

    // Notification messages
    m.insert(
        "notify_ai_completed_body".to_string(),
        "已完成输出，请返回查看。".to_string(),
    );
    // MCP 系统消息
    m.insert("mcp_servers".to_string(), "MCP 服务器".to_string());
    m.insert("mcp_integration_initialized".to_string(), "MCP 集成已初始化".to_string());
    m.insert("mcp_integration_failed".to_string(), "MCP 集成失败".to_string());
    m.insert("mcp_not_available".to_string(), "MCP 集成不可用".to_string());
    m.insert("mcp_no_servers".to_string(), "未配置 MCP 服务器".to_string());
    m.insert("mcp_server_loading".to_string(), "加载中...".to_string());
    m.insert("mcp_connected".to_string(), "已连接".to_string());
    m.insert("mcp_failed_connect".to_string(), "连接失败".to_string());

    // MCP 命令
    m.insert("mcp_status".to_string(), "显示 MCP 服务器状态".to_string());
    m.insert("mcp_tools".to_string(), "列出所有可用工具".to_string());
    m.insert("mcp_tools_server".to_string(), "列出指定服务器的工具".to_string());
    m.insert("mcp_resources".to_string(), "列出所有可用资源".to_string());
    m.insert("mcp_resources_server".to_string(), "列出指定服务器的资源".to_string());
    m.insert("mcp_call_tool".to_string(), "调用工具".to_string());
    m.insert("mcp_read_resource".to_string(), "读取资源".to_string());
    m.insert("mcp_help".to_string(), "显示 MCP 帮助".to_string());

    // MCP 状态消息
    m.insert("mcp_available_tools".to_string(), "可用的 MCP 工具".to_string());
    m.insert("mcp_no_tools".to_string(), "无可用工具".to_string());
    m.insert("mcp_tools_for_server".to_string(), "服务器工具".to_string());
    m.insert("mcp_server_not_found".to_string(), "服务器未找到".to_string());
    m.insert("mcp_failed_get_tools".to_string(), "获取工具失败".to_string());
    m.insert("mcp_calling_tool".to_string(), "正在调用工具".to_string());
    m.insert("mcp_tool_result".to_string(), "结果".to_string());
    m.insert("mcp_tool_failed".to_string(), "工具调用失败".to_string());
    m.insert("mcp_reading_resource".to_string(), "正在读取资源".to_string());
    m.insert("mcp_resource_content".to_string(), "资源内容".to_string());
    m.insert("mcp_resource_failed".to_string(), "资源读取失败".to_string());

    // MCP 提示系统
    m.insert("prompt_interactive_flow".to_string(), "交互式提示流程".to_string());
    m.insert("prompt_available_servers".to_string(), "可用的 MCP 服务器".to_string());
    m.insert("prompt_select_server".to_string(), "选择服务器 ({}-{}) 或 'q' 退出".to_string());
    m.insert("prompt_selected_server".to_string(), "已选择".to_string());
    m.insert("prompt_using_server".to_string(), "使用服务器".to_string());
    m.insert("prompt_getting_prompts".to_string(), "从以下服务器获取提示".to_string());
    m.insert("prompt_failed_get_prompts".to_string(), "获取提示失败".to_string());
    m.insert("prompt_no_prompts".to_string(), "服务器无可用提示".to_string());
    m.insert("prompt_using_prompt".to_string(), "使用提示".to_string());
    m.insert("prompt_available_prompts".to_string(), "可用提示".to_string());
    m.insert("prompt_select_prompt".to_string(), "选择提示 ({}-{}) 或 'q' 退出".to_string());
    m.insert("prompt_selected_prompt".to_string(), "已选择".to_string());
    m.insert("prompt_executing".to_string(), "正在执行提示".to_string());
    m.insert("prompt_collecting_args".to_string(), "收集参数".to_string());
    m.insert("prompt_no_args_required".to_string(), "无需参数".to_string());
    m.insert("prompt_arg_required".to_string(), "此参数为必填项".to_string());
    m.insert("prompt_result".to_string(), "提示结果".to_string());
    m.insert("prompt_completed".to_string(), "提示执行完成".to_string());
    m.insert("prompt_invalid_choice".to_string(), "无效选择，请输入".to_string());

    // MCP 帮助消息
    m.insert("mcp_commands_help".to_string(), "MCP 命令".to_string());
    m.insert("mcp_examples".to_string(), "示例".to_string());
    m.insert("prompt_command_help".to_string(), "提示命令帮助".to_string());
    m.insert("prompt_flow".to_string(), "流程".to_string());
    m.insert("prompt_features".to_string(), "功能特性".to_string());
    m.insert("prompt_note".to_string(), "注意".to_string());
    m.insert("prompt_mcp_compliant".to_string(), "所有提示及其参数均由 MCP 服务器定义。".to_string());
    m.insert("prompt_no_hardcoded".to_string(), "无硬编码提示类型 - 一切均动态发现。".to_string());

    // MCP 错误消息
    m.insert("mcp_tool_error".to_string(), "MCP 工具错误".to_string());
    m.insert("mcp_resource_error".to_string(), "MCP 资源错误".to_string());
    m.insert("mcp_invalid_uri_format".to_string(), "无效的 MCP URI 格式，请使用：mcp://server/resource".to_string());
    m.insert("mcp_tool_executed".to_string(), "MCP 工具已执行".to_string());
    m.insert("mcp_unknown_command".to_string(), "未知的 MCP 命令".to_string());

    // MCP 命令使用消息
    m.insert("mcp_usage_call".to_string(), "用法：mcp call <服务器> <工具> [参数_json]".to_string());
    m.insert("mcp_usage_read".to_string(), "用法：mcp read <服务器> <uri>".to_string());
    m.insert("mcp_available_resources".to_string(), "可用的 MCP 资源".to_string());
    m.insert("mcp_resources_for_server".to_string(), "服务器资源".to_string());
    m.insert("mcp_resource_not_implemented".to_string(), "资源列表功能尚未实现".to_string());
    m.insert("mcp_calling_tool_msg".to_string(), "正在服务器 '{}' 上调用工具 '{}'...".to_string());
    m.insert("mcp_reading_resource_msg".to_string(), "正在从服务器 '{}' 读取资源 '{}'...".to_string());

    // MCP 交互消息
    m.insert("mcp_no_servers_connected".to_string(), "无已连接的 MCP 服务器".to_string());
    m.insert("mcp_getting_prompts_from".to_string(), "正在从 '{}' 获取提示...".to_string());
    m.insert("mcp_invalid_choice_range".to_string(), "无效选择，请输入 1-{} 或 'q'".to_string());
    m.insert("mcp_collecting_args_for".to_string(), "正在为 '{}' 收集参数".to_string());
    m.insert("mcp_prompt_result_header".to_string(), "提示结果".to_string());
    m.insert("mcp_prompt_execution_completed".to_string(), "提示执行已完成".to_string());
    m.insert("mcp_sending_to_ai".to_string(), "正在发送给AI处理...".to_string());
    m.insert("mcp_ai_response_header".to_string(), "AI回复".to_string());
    m.insert("mcp_ai_response_failed".to_string(), "获取AI回复失败".to_string());
    m.insert("mcp_ai_tool_calls".to_string(), "AI想要执行".to_string());
    
    // Prompt selection i18n
    m.insert("prompt_available_prompts".to_string(), "可用提示".to_string());
    m.insert("prompt_select_or_quit".to_string(), "选择提示 (1-{}) 或 'q' 退出".to_string());
    m.insert("prompt_selected".to_string(), "已选择".to_string());
    m.insert("prompt_executing".to_string(), "正在执行提示".to_string());
    m.insert("prompt_collecting_args".to_string(), "正在为以下项目收集参数".to_string());
    m.insert("prompt_result_header".to_string(), "提示结果".to_string());

    m
}

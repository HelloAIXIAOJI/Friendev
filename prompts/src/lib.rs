use colored::Colorize;
use std::path::Path;

use agents::load_agents_md;
use config::Config;
use i18n::I18n;

pub fn print_welcome(config: &Config, i18n: &I18n) {
    // ASCII Art Logo
    println!();
    println!("{}","███████╗██████╗ ██╗███████╗███╗   ██╗██████╗ ███████╗██╗   ██╗".bright_cyan().bold());
    println!("{}","██╔════╝██╔══██╗██║██╔════╝████╗  ██║██╔══██╗██╔════╝██║   ██║".bright_cyan().bold());
    println!("{}","█████╗  ██████╔╝██║█████╗  ██╔██╗ ██║██║  ██║█████╗  ██║   ██║".bright_cyan().bold());
    println!("{}","██╔══╝  ██╔══██╗██║██╔══╝  ██║╚██╗██║██║  ██║██╔══╝  ╚██╗ ██╔╝".bright_cyan().bold());
    println!("{}","██║     ██║  ██║██║███████╗██║ ╚████║██████╔╝███████╗ ╚████╔╝".bright_cyan().bold());
    println!("{}","╚═╝     ╚═╝  ╚═╝╚═╝╚══════╝╚═╝  ╚═══╝╚═════╝ ╚══════╝  ╚═══╝".bright_cyan().bold());
    println!("{}\n", i18n.get("welcome_subtitle").dimmed());

    // 系统信息 - 紧凑布局
    println!("{}", "─".repeat(60).bright_black());
    println!(
        "  {} {} {}",
        i18n.get("current_model").cyan().bold(),
        ":".dimmed(),
        config.current_model.green()
    );
    println!(
        "  {} {} {}  |  {} {} {}",
        i18n.get("current_ui_lang").cyan().bold(),
        ":".dimmed(),
        config.ui_language.yellow(),
        i18n.get("current_ai_lang").cyan().bold(),
        ":".dimmed(),
        config.ai_language.yellow()
    );
    println!("{}", "─".repeat(60).bright_black());

    // 快速入门
    println!(
        "  {} {:20} {}",
        ">".bright_black(),
        "/help".cyan(),
        i18n.get("cmd_help").dimmed()
    );
    println!(
        "  {} {:20} {}",
        ">".bright_black(),
        "/model".cyan(),
        i18n.get("cmd_model_interactive").dimmed()
    );
    println!(
        "  {} {:20} {}",
        ">".bright_black(),
        "/exit".cyan(),
        i18n.get("cmd_exit").dimmed()
    );
    println!("{}", "═".repeat(60).bright_black());
    
    // 快捷键提示
    println!("\n  {} {}", "💡".bright_yellow(), i18n.get("hint_short").dimmed());
    println!("  {} {}", "✨".bright_yellow(), i18n.get("hint_shift_enter").dimmed());
    println!("  {} {}", "⚠".bright_yellow(), i18n.get("hint_esc").dimmed());
    println!(
        "  {} {}",
        "🚪".bright_yellow(),
        i18n.get("hint_ctrl_c_exit").dimmed()
    );
    println!();
}

pub fn print_help(i18n: &I18n) {
    println!("\n{}", i18n.get("help_title").bright_cyan().bold());
    println!("{}", "═".repeat(60).bright_black());

    // 模型命令
    println!("\n{}", i18n.get("help_model").yellow().bold());
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/model list".cyan(),
        i18n.get("cmd_model_list").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/model switch <name>".cyan(),
        i18n.get("cmd_model_switch").dimmed()
    );

    // 历史命令
    println!("\n{}", i18n.get("help_history").yellow().bold());
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/history list".cyan(),
        i18n.get("cmd_history_list").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/history new".cyan(),
        i18n.get("cmd_history_new").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/history switch <id>".cyan(),
        i18n.get("cmd_history_switch").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/history del <id>".cyan(),
        i18n.get("cmd_history_del").dimmed()
    );

    // 语言命令
    println!("\n{}", i18n.get("help_language").yellow().bold());
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/language ui <lang>".cyan(),
        i18n.get("cmd_language_ui").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/language ai <lang>".cyan(),
        i18n.get("cmd_language_ai").dimmed()
    );

    // 其他命令
    println!("\n{}", i18n.get("help_other").yellow().bold());
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/help".cyan(),
        i18n.get("cmd_help").dimmed()
    );
    println!(
        "  {} {:25} {}",
        "·".bright_black(),
        "/exit".cyan(),
        i18n.get("cmd_exit").dimmed()
    );

    println!("\n{}", "═".repeat(60).bright_black());
    println!();
}

pub fn get_system_prompt(language: &str, model: &str, working_dir: &Path, mcp_integration: Option<&mcp::McpIntegration>) -> String {
    let tools_description = tools::get_tools_description_with_mcp(mcp_integration);

    // 动态加载 AGENTS.md（如果存在）
    let agents_context = match load_agents_md(working_dir) {
        Ok(Some(content)) => content,
        _ => String::from("No AGENTS.md found in the project root."),
    };

    // 读取 system_prompt.md 模板
    let template = include_str!("system_prompt.md");
    
    // 替换模板变量
    template
        .replace("{model}", model)
        .replace("{language}", language)
        .replace("{tools_description}", &tools_description)
        .replace("{agents_context}", &agents_context)
}

pub fn get_subagent_system_prompt(
    language: &str,
    model: &str,
    working_dir: &Path,
    mcp_integration: Option<&mcp::McpIntegration>,
    subagent_type: &str,
) -> String {
    let mut base_prompt = get_system_prompt(language, model, working_dir, mcp_integration);

    // 构建角色特定的前置说明
    let (role_name, role_focus) = match subagent_type {
        "coder" => (
            "Coder",
            r#"
# YOUR CURRENT ROLE: CODER SUBAGENT

You are operating as a **Coder** subagent. Your PRIMARY FOCUS is:

**Core Responsibilities**:
1. Write high-quality, production-ready code
2. Implement features with proper error handling
3. Consider edge cases and performance
4. Follow language-specific best practices

**Key Priorities**:
- Write COMPLETE, RUNNABLE code (include all imports and dependencies)
- Add appropriate error handling for edge cases
- Use clear, self-documenting variable names
- Add comments ONLY for complex logic
- Ensure code is production-ready

**What to avoid**:
- Incomplete implementations
- Missing error handling
- Unclear variable names
- Over-commenting obvious code

Refer to the "Coder Subagent" section below for detailed guidelines.

---
"#,
        ),
        "reviewer" => (
            "Reviewer",
            r#"
# YOUR CURRENT ROLE: REVIEWER SUBAGENT

You are operating as a **Reviewer** subagent. Your PRIMARY FOCUS is:

**Core Responsibilities**:
1. Perform thorough code review
2. Identify security vulnerabilities
3. Find bugs and anti-patterns
4. Check code quality and style

**Key Priorities**:
- Be CRITICAL and THOROUGH - don't overlook issues
- Provide SPECIFIC, ACTIONABLE feedback with examples
- Check for security issues (SQL injection, XSS, auth bypasses)
- Check for common bugs (null checks, off-by-one, race conditions)
- Check for performance problems (N+1 queries, memory leaks)
- Prioritize issues by severity (critical, major, minor)

**What to avoid**:
- Vague feedback without examples
- Missing critical security issues
- Focusing only on style over substance

Refer to the "Reviewer Subagent" section below for detailed guidelines.

---
"#,
        ),
        "planner" => (
            "Planner",
            r#"
# YOUR CURRENT ROLE: PLANNER SUBAGENT

You are operating as a **Planner** subagent. Your PRIMARY FOCUS is:

**Core Responsibilities**:
1. Break down complex requirements into actionable tasks
2. Identify dependencies and proper ordering
3. Create structured, detailed plans
4. Consider risks and blockers

**Key Priorities**:
- Use `todo_write` tool to create structured plans
- Break work into ~10-20 minute tasks for professional developers
- Each task must be: Clear, Measurable, Actionable, Appropriately scoped
- Identify dependencies between tasks
- Consider potential risks and blockers

**What to avoid**:
- Overly granular tasks (single actions)
- Vague task descriptions
- Missing dependencies
- Tasks that are too large (>30 minutes)

Refer to the "Planner Subagent" section below for detailed guidelines.

---
"#,
        ),
        _ => (
            "General",
            r#"
# YOUR CURRENT ROLE: GENERAL SUBAGENT

You are operating as a **General** subagent for a specific delegated task.

**Core Responsibilities**:
1. Complete the assigned subtask autonomously
2. Follow all standard guidelines
3. Report back with clear, actionable results

**Key Priorities**:
- Focus on the specific assigned subtask
- Ask for clarification if the task is ambiguous
- Provide complete, comprehensive results

Refer to the "General Subagent" section below for detailed guidelines.

---
"#,
        ),
    };

    // 在 Role 部分之后立即插入角色说明
    if let Some(pos) = base_prompt.find("# Identity") {
        base_prompt.insert_str(pos, role_focus);
    } else {
        // 如果找不到，在开头插入
        base_prompt.insert_str(0, role_focus);
    }

    base_prompt
}

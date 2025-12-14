use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;
use std::fs;
use std::collections::HashMap;
use colored::Colorize;
use mlua::prelude::*;
use futures_util::future::BoxFuture;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum HookType {
    Startup,
    PreCommand,
    PostCommand,
    PreApproval,
    PostApproval,
    SessionStart,
    SessionEnd,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HookStep {
    // 简写形式：直接字符串（默认为 shell 命令）
    Simple(String),
    // 完整形式：对象
    Complex(HookStepConfig),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookStepConfig {
    pub name: Option<String>,
    #[serde(default)]
    pub run: Option<String>, // Shell 命令
    #[serde(default)]
    pub lua: Option<String>, // 内联 Lua 代码
    #[serde(default)]
    pub command: Option<String>, // Friendev Slash Command
    #[serde(default)]
    pub uses: Option<String>, // 引用 Lua 脚本文件
    #[serde(default)]
    pub shell: Option<String>, // 显式指定 shell
    #[serde(default)]
    pub env: HashMap<String, String>, // 步骤特定的环境变量
    #[serde(default = "default_true")]
    pub continue_on_error: bool,
}

fn default_true() -> bool { true }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookConfig {
    // 支持两种格式：
    // 1. "pre_command": ["echo hi", { "uses": "..." }]
    // 2. "pre_command": { "steps": [...] } - 更接近 GitHub Actions 顶层结构
    #[serde(flatten)]
    pub hooks: HashMap<HookType, HookDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HookDefinition {
    List(Vec<HookStep>),
    Detailed { steps: Vec<HookStep> },
}

impl HookDefinition {
    fn steps(&self) -> &[HookStep] {
        match self {
            HookDefinition::List(steps) => steps,
            HookDefinition::Detailed { steps } => steps,
        }
    }
}

pub struct HookContext {
    pub working_dir: PathBuf,
    pub env_vars: HashMap<String, String>,
}

impl HookContext {
    pub fn new(working_dir: PathBuf) -> Self {
        Self {
            working_dir,
            env_vars: HashMap::new(),
        }
    }

    pub fn with_env(mut self, key: &str, value: &str) -> Self {
        self.env_vars.insert(key.to_string(), value.to_string());
        self
    }
}

pub type FriendevCommandRunner = dyn Fn(&str) -> BoxFuture<'static, Result<()>> + Send + Sync;

pub async fn execute_hook(
    hook_type: HookType, 
    context: &HookContext,
    command_runner: Option<&FriendevCommandRunner>,
) -> Result<()> {
    let hook_config_path = context.working_dir.join(".friendev").join("hooks.json");
    
    // Early return if file doesn't exist
    if !hook_config_path.exists() {
        return Ok(());
    }

    // Read and parse config
    let config_content = fs::read_to_string(&hook_config_path)?;
    #[derive(Deserialize)]
    struct RootConfig {
        hooks: HashMap<HookType, HookDefinition>,
    }
    
    let config: RootConfig = match serde_json::from_str(&config_content) {
        Ok(c) => c,
        Err(e) => {
            // Silently ignore parse errors for PreCommand/PostCommand to avoid spam
            if !matches!(hook_type, HookType::PreCommand | HookType::PostCommand) {
                eprintln!("{}", format!("Warning: Failed to parse hooks.json: {}", e).yellow());
            }
            return Ok(());
        }
    };

    // Check if hooks exist for this type
    if let Some(definition) = config.hooks.get(&hook_type) {
        let steps = definition.steps();
        if !steps.is_empty() {
            // Only print for hooks that actually have steps
            if matches!(hook_type, HookType::Startup | HookType::SessionStart | HookType::SessionEnd) {
                println!("{}", format!("Running {:?} hooks...", hook_type).bright_black());
            }
            
            for (idx, step) in steps.iter().enumerate() {
                if let Err(e) = execute_step(step, context, idx, command_runner).await {
                    // Only show errors for non-PreCommand/PostCommand to avoid spam
                    if !matches!(hook_type, HookType::PreCommand | HookType::PostCommand) {
                        eprintln!("{}", format!("Hook step failed: {}", e).red());
                    }
                }
            }
        }
    }

    Ok(())
}

async fn execute_step(
    step: &HookStep, 
    context: &HookContext, 
    idx: usize,
    command_runner: Option<&FriendevCommandRunner>,
) -> Result<()> {
    match step {
        HookStep::Simple(cmd_str) => {
            // Legacy/Simple support: try to guess if it looks like a file or shell command
            // But for GitHub Actions style, simple strings are usually shell commands.
            // We will stick to shell execution for simple strings, unless it ends in .lua?
            // Let's keep it simple: Simple strings are SHELL commands.
            // Unless it starts with "lua:" (backward compat)
            
            if cmd_str.starts_with("lua:") {
                let lua_code = &cmd_str[4..];
                execute_lua_script(lua_code, context, &HashMap::new())?;
            } else {
                execute_shell_command(cmd_str, None, context, &HashMap::new())?;
            }
        }
        HookStep::Complex(config) => {
            if let Some(name) = &config.name {
                println!("{}", format!("  Step {}: {}", idx + 1, name).cyan());
            } else {
                println!("{}", format!("  Step {}", idx + 1).bright_black());
            }

            // Merge env vars
            let mut step_env = context.env_vars.clone();
            for (k, v) in &config.env {
                step_env.insert(k.clone(), v.clone());
            }
            // Update context with merged env for this step (conceptually)
            // Actually execute_XXX helpers just take map.

            if let Some(lua_code) = &config.lua {
                execute_lua_script(lua_code, context, &config.env)?;
            } else if let Some(script_path) = &config.uses {
                // Load file content
                let path = resolve_script_path(script_path, &context.working_dir);
                if path.exists() {
                    let content = fs::read_to_string(&path)?;
                    if script_path.ends_with(".lua") {
                        execute_lua_script(&content, context, &config.env)?;
                    } else {
                        // Maybe support other script types later?
                        // For now, assume uses points to lua or executable script
                        // If executable, run it?
                        // Let's stick to lua for 'uses' currently as requested.
                        execute_lua_script(&content, context, &config.env)?;
                    }
                } else {
                    anyhow::bail!("Script file not found: {}", script_path);
                }
            } else if let Some(cmd) = &config.command {
                if let Some(runner) = command_runner {
                    println!("{}", format!("    Executing Friendev command: {}", cmd).bright_black());
                    if let Err(e) = runner(cmd).await {
                        if !config.continue_on_error {
                            return Err(e);
                        }
                        eprintln!("{}", format!("    Command failed (continue_on_error=true): {}", e).yellow());
                    }
                } else {
                    eprintln!("{}", "    Warning: Friendev commands are not supported in this hook context.".yellow());
                }
            }
        }
    }
    Ok(())
}

fn resolve_script_path(path_str: &str, working_dir: &std::path::Path) -> PathBuf {
    if working_dir.join(".friendev").join(path_str).exists() {
        working_dir.join(".friendev").join(path_str)
    } else {
        working_dir.join(path_str)
    }
}

fn execute_shell_command(
    cmd_str: &str, 
    shell: Option<&str>, 
    context: &HookContext, 
    step_env: &HashMap<String, String>
) -> Result<()> {
    let mut merged_env = context.env_vars.clone();
    for (k, v) in step_env {
        merged_env.insert(k.clone(), v.clone());
    }

    let mut command;
    
    if let Some(sh) = shell {
        command = Command::new(sh);
        if sh.contains("pwsh") || sh.contains("powershell") {
            command.arg("-Command");
        } else {
            command.arg("-c");
        }
        command.arg(cmd_str);
    } else {
        // Default shell detection
        if cfg!(target_os = "windows") {
            command = Command::new("cmd");
            command.arg("/C");
            command.arg(cmd_str);
        } else {
            command = Command::new("sh");
            command.arg("-c");
            command.arg(cmd_str);
        }
    }

    command.current_dir(&context.working_dir)
           .envs(&merged_env);

    let status = command.status()?;
    if !status.success() {
        anyhow::bail!("Command failed with status: {}", status);
    }
    Ok(())
}

fn execute_lua_script(script: &str, context: &HookContext, step_env: &HashMap<String, String>) -> LuaResult<()> {
    let lua = Lua::new();

    // Inject context into Lua globals
    let globals = lua.globals();
    
    let env_table = lua.create_table()?;
    // Base env
    for (k, v) in &context.env_vars {
        env_table.set(k.clone(), v.clone())?;
    }
    // Step-specific env overrides
    for (k, v) in step_env {
        env_table.set(k.clone(), v.clone())?;
    }
    globals.set("env", env_table)?;
    
    globals.set("working_dir", context.working_dir.to_string_lossy().to_string())?;

    // Helper print function
    let print = lua.create_function(|_, msg: String| {
        println!("{}", msg);
        Ok(())
    })?;
    globals.set("print", print)?;

    lua.load(script).exec()?;

    Ok(())
}

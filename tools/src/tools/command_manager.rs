use anyhow::Result;
use dirs;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

/// 默认的总是需要确认的命令
const DEFAULT_ALWAYS_APPROVE_COMMANDS: &[&str] = &["rm", "del", "rmdir", "format", "fdisk"];

/// 命令配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandConfig {
    /// 总是需要确认的命令列表
    pub always_approve_commands: HashSet<String>,
    /// 运行中的后台命令
    pub running_commands: Vec<BackgroundCommand>,
}

/// 后台运行命令信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundCommand {
    /// 命令ID
    pub id: String,
    /// 执行的命令
    pub command: String,
    /// 启动时间
    pub start_time: chrono::DateTime<chrono::Utc>,
    /// 命令状态：running, completed, failed
    pub status: String,
    /// 命令退出码（如果已完成）
    pub exit_code: Option<i32>,
    /// 命令输出（如果已收集）
    pub output: Option<String>,
}

impl Default for CommandConfig {
    fn default() -> Self {
        let mut always_approve_commands = HashSet::new();
        for cmd in DEFAULT_ALWAYS_APPROVE_COMMANDS {
            always_approve_commands.insert(cmd.to_string());
        }

        Self {
            always_approve_commands,
            running_commands: Vec::new(),
        }
    }
}

impl CommandConfig {
    /// 加载配置
    pub fn load() -> Result<Self> {
        let path = get_config_path();

        if path.exists() {
            let content = fs::read_to_string(&path)?;
            let config: CommandConfig = serde_json::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    /// 保存配置
    pub fn save(&self) -> Result<()> {
        let path = get_config_path();

        // 确保目录存在
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&path, content)?;
        Ok(())
    }

    /// 添加总是需要确认的命令
    pub fn add_always_approve_command(&mut self, command: &str) -> bool {
        self.always_approve_commands.insert(command.to_string())
    }

    /// 移除总是需要确认的命令
    pub fn remove_always_approve_command(&mut self, command: &str) -> bool {
        self.always_approve_commands.remove(command)
    }

    /// 检查命令是否总是需要确认
    pub fn needs_approval(&self, command: &str) -> bool {
        // 提取主命令（第一个词）
        let main_command = command
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_lowercase();
        self.always_approve_commands.contains(&main_command)
    }

    /// 添加后台命令
    pub fn add_background_command(&mut self, cmd: BackgroundCommand) {
        self.running_commands.push(cmd);
    }

    /// 更新后台命令状态
    pub fn update_background_command<F>(&mut self, id: &str, updater: F) -> bool
    where
        F: FnOnce(&mut BackgroundCommand),
    {
        if let Some(cmd) = self.running_commands.iter_mut().find(|c| c.id == id) {
            updater(cmd);
            true
        } else {
            false
        }
    }

    /// 获取后台命令
    pub fn get_background_command(&self, id: &str) -> Option<&BackgroundCommand> {
        self.running_commands.iter().find(|c| c.id == id)
    }

    /// 列出所有总是需要确认的命令
    pub fn list_always_approve_commands(&self) -> Vec<String> {
        let mut commands: Vec<String> = self.always_approve_commands.iter().cloned().collect();
        commands.sort();
        commands
    }
}

/// 获取配置文件路径
fn get_config_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("friendev");
    path.push("commands.json");
    path
}

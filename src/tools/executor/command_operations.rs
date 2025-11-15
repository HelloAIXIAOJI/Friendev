use anyhow::Result;

use crate::tools::types::{ToolResult, is_action_approved, approve_action_for_session};
use crate::tools::args::RunCommandArgs;
use crate::ui::prompt_approval;

pub async fn execute_run_command(
    arguments: &str,
    require_approval: bool,
) -> Result<ToolResult> {
    let args: RunCommandArgs = serde_json::from_str(arguments)?;
    
    // 加载命令配置
    let config = crate::tools::command_manager::CommandConfig::load()?;
    
    // 检查是否需要审批
    let needs_approval = require_approval || config.needs_approval(&args.command);
    
    if needs_approval && !is_action_approved("run_command") {
        // 提取主命令用于显示
        let main_command = args.command.split_whitespace().next().unwrap_or("");
        
        let (approved, always, view_details) = prompt_approval(
            "RunCommand",
            &format!("{}", args.command),
            Some(&format!("Command: {}\nMode: {}", main_command, if args.background { "background" } else { "foreground" }))
        )?;
        
        if view_details {
            let continue_operation = crate::ui::show_detailed_content(
                "RunCommand",
                &format!("Command: {}", args.command),
                &format!("Full command:\n{}\n\nThis command will be executed in {} mode.", 
                    args.command, 
                    if args.background { "background" } else { "foreground" })
            )?;
            
            if !continue_operation {
                return Ok(ToolResult::error("User cancelled the operation".to_string()));
            }
        }
        
        if !approved {
            return Ok(ToolResult::error("User rejected the operation".to_string()));
        }
        
        if always {
            approve_action_for_session("run_command");
        }
    }
    
    if args.background {
        execute_background_command(args, config).await
    } else {
        execute_foreground_command(args).await
    }
}

async fn execute_background_command(
    args: RunCommandArgs,
    mut config: crate::tools::command_manager::CommandConfig,
) -> Result<ToolResult> {
    use uuid::Uuid;
    use tokio::process::Command as TokioCommand;
    
    let run_id = Uuid::new_v4().to_string();
    let run_id_for_async = run_id.clone();
    let command_for_async = args.command.clone();
    
    // 创建后台命令
    let bg_cmd = crate::tools::command_manager::BackgroundCommand {
        id: run_id.clone(),
        command: args.command.clone(),
        start_time: chrono::Utc::now(),
        status: "running".to_string(),
        exit_code: None,
        output: None,
    };
    
    // 保存到配置
    config.add_background_command(bg_cmd);
    config.save()?;
    
    // 在后台启动命令
    tokio::spawn(async move {
        let mut cmd = if cfg!(target_os = "windows") {
            TokioCommand::new("cmd")
        } else {
            TokioCommand::new("sh")
        };
        
        if cfg!(target_os = "windows") {
            cmd.arg("/C");
        } else {
            cmd.arg("-c");
        }
        
        cmd.arg(&command_for_async);
        
        match cmd.output().await {
            Ok(output) => {
                // 更新命令状态
                let mut config = match crate::tools::command_manager::CommandConfig::load() {
                    Ok(c) => c,
                    Err(_) => return, // 如果无法加载配置，直接返回
                };
                
                let status = if output.status.success() { "completed" } else { "failed" };
                let exit_code = output.status.code();
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                
                let combined_output = if !stdout.is_empty() && !stderr.is_empty() {
                    format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
                } else if !stdout.is_empty() {
                    stdout
                } else {
                    stderr
                };
                
                config.update_background_command(&run_id_for_async, |cmd| {
                    cmd.status = status.to_string();
                    cmd.exit_code = exit_code;
                    cmd.output = Some(combined_output);
                });
                
                let _ = config.save();
            }
            Err(_) => {
                // 更新命令状态为失败
                let mut config = match crate::tools::command_manager::CommandConfig::load() {
                    Ok(c) => c,
                    Err(_) => return,
                };
                
                config.update_background_command(&run_id_for_async, |cmd| {
                    cmd.status = "failed".to_string();
                    cmd.exit_code = None;
                    cmd.output = Some("Failed to execute command".to_string());
                });
                
                let _ = config.save();
            }
        }
    });
    
    let brief = format!("Started background command: {}", run_id);
    let output = format!("Command started in background\nRun ID: {}\nCommand: {}\n\nUse /runcommand info {{}} to check status", run_id, args.command);
    
    Ok(ToolResult::ok(brief, output))
}

async fn execute_foreground_command(args: RunCommandArgs) -> Result<ToolResult> {
    use std::process::Command;
    
    let mut cmd = if cfg!(target_os = "windows") {
        Command::new("cmd")
    } else {
        Command::new("sh")
    };
    
    if cfg!(target_os = "windows") {
        cmd.arg("/C");
    } else {
        cmd.arg("-c");
    }
    
    cmd.arg(&args.command);
    
    match cmd.output() {
        Ok(output) => {
            let status = if output.status.success() { "success" } else { "failed" };
            let exit_code = output.status.code().unwrap_or(-1);
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            
            let combined_output = if !stdout.is_empty() && !stderr.is_empty() {
                format!("STDOUT:\n{}\nSTDERR:\n{}", stdout, stderr)
            } else if !stdout.is_empty() {
                stdout
            } else {
                stderr
            };
            
            let brief = format!("Command executed: {} (exit: {})", status, exit_code);
            let output_text = format!("Command: {}\nExit code: {}\nStatus: {}\n\nOutput:\n{}", args.command, exit_code, status, combined_output);
            
            Ok(ToolResult::ok(brief, output_text))
        }
        Err(e) => {
            let brief = format!("Failed to execute command: {}", e);
            
            Ok(ToolResult::error(brief))
        }
    }
}

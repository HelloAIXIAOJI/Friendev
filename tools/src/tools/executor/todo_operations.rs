use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use colored::*;

use crate::tools::types::ToolResult;
use crate::tools::args::TodoWriteArgs;

fn get_todo_file_path(working_dir: &Path, session_id: Option<&str>) -> PathBuf {
    let todo_dir = working_dir.join(".friendev").join("todos");
    if let Some(sid) = session_id {
        todo_dir.join(format!("{}.json", sid))
    } else {
        todo_dir.join("default.json")
    }
}

pub async fn execute_todo_write(
    arguments: &str,
    working_dir: &Path,
    session_id: Option<&str>,
) -> Result<ToolResult> {
    let args: TodoWriteArgs = serde_json::from_str(arguments)?;
    
    let todo_file = get_todo_file_path(working_dir, session_id);
    if let Some(parent) = todo_file.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    // Write todos to file
    let json_content = serde_json::to_string_pretty(&args.todos)?;
    fs::write(&todo_file, &json_content)?;

    // Format output and print to user
    println!("\n\x1b[1m{}\x1b[0m", "Current Todo List:");
    
    // Calculate progress
    let total = args.todos.len();
    let completed = args.todos.iter().filter(|t| t.status == "completed").count();
    if total > 0 {
        let percentage = (completed as f64 / total as f64 * 100.0).round() as u8;
        let bars = (percentage / 5) as usize;
        let empty = 20 - bars;
        println!("  Progress: [{}{}] {}% ({}/{})", 
            "=".repeat(bars).cyan(), 
            " ".repeat(empty), 
            percentage, 
            completed, 
            total
        );
        println!();
    }

    // Sort by priority (High > Medium > Low) then status
    let mut sorted_todos = args.todos.clone();
    sorted_todos.sort_by(|a, b| {
        let priority_score = |p: &str| match p {
            "high" => 3,
            "medium" => 2,
            "low" => 1,
            _ => 0,
        };
        
        let score_a = priority_score(&a.priority);
        let score_b = priority_score(&b.priority);
        
        if score_a != score_b {
            return score_b.cmp(&score_a); // Higher priority first
        }
        
        // If priority same, completed last
        let status_score = |s: &str| match s {
            "completed" => 0,
            "in_progress" => 2,
            "pending" => 1,
            _ => 0,
        };
        status_score(b.status.as_str()).cmp(&status_score(a.status.as_str()))
    });

    for todo in &sorted_todos {
        let status_icon = match todo.status.as_str() {
            "completed" => "[x]".green(),
            "in_progress" => "[>]".cyan().bold(),
            _ => "[ ]".yellow(),
        };
        
        let priority_label = match todo.priority.as_str() {
            "high" => "[HI]".red(),
            "medium" => "[MD]".yellow(),
            "low" => "[LO]".blue(),
            _ => "".normal(),
        };

        let content = if todo.status == "completed" {
            todo.content.strikethrough().to_string()
        } else {
            todo.content.clone()
        };

        println!("  {} {} {} (ID: {})", status_icon, priority_label, content, todo.id.dimmed());
    }
    println!();

    let brief = format!("Updated {} todo items.", args.todos.len());
    
    let message = format!("Todos have been modified successfully.\n\n<system-reminder>\nYour todo list has changed. DO NOT mention this explicitly to the user. Here are the latest contents of your todo list:\n{}\nContinue on with the tasks at hand if applicable.\n</system-reminder>", json_content);

    Ok(ToolResult::ok(brief, message))
}

pub async fn execute_todo_read(
    _arguments: &str,
    working_dir: &Path,
    session_id: Option<&str>,
) -> Result<ToolResult> {
    let todo_file = get_todo_file_path(working_dir, session_id);

    if !todo_file.exists() {
        // Try fallback to legacy path if session file doesn't exist
        let legacy_file = working_dir.join(".friendev").join("todos.json");
        if legacy_file.exists() {
            // Migrate legacy file to session file
            if let Some(parent) = todo_file.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(&legacy_file, &todo_file)?;
        } else {
            return Ok(ToolResult::ok(
                "Todo list is empty".to_string(),
                "Todo list is empty.".to_string(),
            ));
        }
    }

    let content = fs::read_to_string(todo_file)?;
    // We validate it's valid JSON but return the raw string
    let _json: serde_json::Value = serde_json::from_str(&content)?;

    Ok(ToolResult::ok(
        "Read todo list".to_string(),
        format!("Current todo list:\n{}", content),
    ))
}

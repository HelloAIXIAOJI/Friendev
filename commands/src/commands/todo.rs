use anyhow::Result;
use colored::*;
use serde::Deserialize;
use std::fs;

use i18n::I18n;

#[derive(Debug, Deserialize)]
struct TodoItem {
    id: String,
    content: String,
    status: String,
    #[serde(default)]
    priority: String,
}

pub fn handle_todo_command(_parts: &[&str], i18n: &I18n) -> Result<()> {
    let working_dir = std::env::current_dir()?;
    let todo_file = working_dir.join(".friendev").join("todos.json");

    if !todo_file.exists() {
        println!("\n  {}\n", i18n.get("todo_list_empty").yellow());
        return Ok(());
    }

    let content = fs::read_to_string(todo_file)?;
    let todos: Vec<TodoItem> = match serde_json::from_str(&content) {
        Ok(t) => t,
        Err(_) => {
            println!("\n  {}\n", i18n.get("todo_file_corrupt").red());
            return Ok(());
        }
    };

    if todos.is_empty() {
        println!("\n  {}\n", i18n.get("todo_list_empty").yellow());
        return Ok(());
    }

    println!("\n\x1b[1m{}\x1b[0m", "Current Todo List:");
    
    // Sort by priority (High > Medium > Low) then status
    let mut sorted_todos = todos;
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
            return score_b.cmp(&score_a);
        }
        
        let status_score = |s: &str| match s {
            "completed" => 0,
            "in_progress" => 2,
            "pending" => 1,
            _ => 0,
        };
        status_score(b.status.as_str()).cmp(&status_score(a.status.as_str()))
    });

    for todo in sorted_todos {
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

    Ok(())
}

fn print_todos(_todos: &[TodoItem]) {
    // Legacy function, kept for compatibility but unused.
}

use anyhow::Result;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Instant;
use tools::tools::indexer::Indexer;
use tools::tools::executor::parser;
use i18n::I18n;

pub async fn handle_index_command(args: Vec<&str>, i18n: &I18n) -> Result<()> {
    if args.is_empty() {
        println!("{}", i18n.get("index_usage_header").yellow());
        println!("{}", i18n.get("index_usage_outline"));
        println!("{}", i18n.get("index_usage_outline_all"));
        return Ok(());
    }

    match args[0] {
        "outline" => {
            let mut full_rebuild = false;
            
            for arg in &args[1..] {
                if *arg == "all" {
                    full_rebuild = true;
                }
            }
            
            // Parse flags properly
            let has_ts_flag = args.iter().any(|&a| a == "--ts");
            let has_lsp_flag = args.iter().any(|&a| a == "--lsp");
            
            let (use_tree_sitter, use_lsp) = if has_ts_flag || has_lsp_flag {
                (has_ts_flag, has_lsp_flag)
            } else {
                // Default both on
                (true, true)
            };

            let start_msg_key = if full_rebuild { "index_start_full" } else { "index_start_incremental" };
            
            println!("{}", i18n.get(start_msg_key).cyan().bold());
            let start_time = Instant::now();
            
            let current_dir = std::env::current_dir()?;
            let indexer = Indexer::new(&current_dir)?;
            
            use ignore::WalkBuilder;
            let walker = WalkBuilder::new(&current_dir).build();
            
            let mut files_to_process = Vec::new();
            
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(ProgressStyle::default_spinner()
                .template("{spinner:.green} Scanning files...")
                .unwrap());
            
            for result in walker {
                if let Ok(entry) = result {
                    if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                        let path = entry.path().to_path_buf();
                        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                            if parser::is_supported_extension(ext) {
                                files_to_process.push(path);
                            }
                        }
                    }
                }
                spinner.tick();
            }
            spinner.finish_and_clear();

            let total_files = files_to_process.len();
            if total_files == 0 {
                println!("{}", i18n.get("index_no_files").yellow());
                return Ok(());
            }

            println!("{}", i18n.get("index_found_files").replace("{}", &total_files.to_string()).dimmed());

            let pb = ProgressBar::new(total_files as u64);
            pb.set_style(ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta}) {msg}")
                .unwrap()
                .progress_chars("#>-"));

            let mut success_count = 0;
            let mut error_count = 0;
            
            for path in files_to_process {
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                
                match indexer.index_file(&path, &current_dir, use_tree_sitter, use_lsp).await {
                    Ok(source) => {
                        success_count += 1;
                        pb.set_message(format!("{} [{}]", file_name, source));
                    },
                    Err(_) => {
                        error_count += 1;
                        pb.set_message(format!("{} [failed]", file_name));
                    }
                }
                pb.inc(1);
            }
            
            pb.finish_with_message("Done");

            use std::process::Command;
            if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
                if output.status.success() {
                    let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    let _ = indexer.set_last_commit(&hash);
                }
            }
            
            let duration = start_time.elapsed();
            println!();
            println!("{}", i18n.get("index_complete").replace("{:.2?}", &format!("{:.2?}", duration)).green().bold());
            println!("{}", i18n.get("index_stat_processed").replace("{}", &total_files.to_string()));
            println!("{}", i18n.get("index_stat_indexed").replace("{}", &success_count.to_string()).green());
            if error_count > 0 {
                println!("{}", i18n.get("index_stat_failed").replace("{}", &error_count.to_string()).red());
            }
            println!();
        }
        _ => {
            println!("{}", i18n.get("index_unknown_subcommand").replace("{}", args[0]).red());
        }
    }

    Ok(())
}

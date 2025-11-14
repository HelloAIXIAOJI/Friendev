use anyhow::Result;
use std::fs;
use std::path::Path;
use regex::Regex;

use crate::tools::types::{ToolResult, is_action_approved, approve_action_for_session};
use crate::tools::args::{FileListArgs, FileReadArgs, FileWriteArgs, FileReplaceArgs, SearchArgs, FileDiffEditArgs, RunCommandArgs};
use crate::ui::prompt_approval;

// 限制max_results到20以内
fn limit_results(max: usize) -> usize {
    std::cmp::min(std::cmp::max(1, max), 20)
}

pub async fn execute_tool(name: &str, arguments: &str, working_dir: &Path, require_approval: bool) -> Result<ToolResult> {
    match name {
        "file_list" => {
            let args: FileListArgs = serde_json::from_str(arguments)
                .unwrap_or(FileListArgs { path: None });
            
            let target_path = if let Some(path) = args.path {
                let p = Path::new(&path);
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    working_dir.join(p)
                }
            } else {
                working_dir.to_path_buf()
            };

            if !target_path.exists() {
                return Ok(ToolResult::error(format!("路径不存在: {}", target_path.display())));
            }

            if !target_path.is_dir() {
                return Ok(ToolResult::error(format!("不是目录: {}", target_path.display())));
            }

            let mut items = Vec::new();
            for entry in fs::read_dir(&target_path)? {
                let entry = entry?;
                let path = entry.path();
                let name = entry.file_name().to_string_lossy().to_string();
                let item_type = if path.is_dir() { "目录" } else { "文件" };
                
                let metadata = entry.metadata()?;
                let size = if metadata.is_file() {
                    crate::tools::utils::format_size(metadata.len())
                } else {
                    "-".to_string()
                };

                items.push(format!("{} [{}] ({})", name, item_type, size));
            }

            items.sort();

            let brief = if items.is_empty() {
                format!("目录为空")
            } else {
                format!("列出 {} 项", items.len())
            };

            let output = format!(
                "目录: {}\n共 {} 项:\n\n{}",
                target_path.display(),
                items.len(),
                items.join("\n")
            );

            Ok(ToolResult::ok(brief, output))
        }
        "file_read" => {
            let args: FileReadArgs = serde_json::from_str(arguments)?;
            
            let target_path = {
                let p = Path::new(&args.path);
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    working_dir.join(p)
                }
            };
            
            if !target_path.exists() {
                return Ok(ToolResult::error(format!("文件不存在: {}", target_path.display())));
            }
            
            if !target_path.is_file() {
                return Ok(ToolResult::error(format!("不是文件: {}", target_path.display())));
            }
            
            let content = fs::read_to_string(&target_path)?;
            let lines = content.lines().count();
            let bytes = content.len();
            
            let brief = format!("读取 {} 行, {} 字节", lines, bytes);
            let output = format!("文件: {}\n内容:\n{}", target_path.display(), content);
            
            Ok(ToolResult::ok(brief, output))
        }
        "file_write" => {
            let args: FileWriteArgs = serde_json::from_str(arguments)?;
            
            let target_path = {
                let p = Path::new(&args.path);
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    working_dir.join(p)
                }
            };
            
            // 验证 mode 参数
            let mode = args.mode.as_str();
            if mode != "overwrite" && mode != "append" {
                return Ok(ToolResult::error(format!("无效的写入模式: {}，只支持 'overwrite' 或 'append'", mode)));
            }
            
            // 危险操作：需要用户确认
            if require_approval && !is_action_approved("file_write") {
                let action_desc = if mode == "append" {
                    format!("追加到文件: {}", target_path.display())
                } else {
                    format!("覆盖文件: {}", target_path.display())
                };
                
                let (approved, always, view_details) = prompt_approval(
                    "WriteFile",
                    &action_desc,
                    Some(&args.content)  // 传递内容预览
                )?;
                
                // 如果用户选择查看详细信息
                if view_details {
                    let continue_operation = crate::ui::show_detailed_content(
                        "WriteFile",
                        &target_path.display().to_string(),
                        &args.content
                    )?;
                    
                    if !continue_operation {
                        return Ok(ToolResult::error("用户取消了该操作".to_string()));
                    }
                }
                
                if !approved {
                    return Ok(ToolResult::error("用户拒绝了该操作".to_string()));
                }
                
                // 如果用户选择 Always，保存状态
                if always {
                    approve_action_for_session("file_write");
                }
            }
            
            // 创建父目录（如果不存在）
            if let Some(parent) = target_path.parent() {
                fs::create_dir_all(parent)?;
            }
            
            // 根据模式写入或追加
            if mode == "append" {
                // 追加模式
                use std::fs::OpenOptions;
                use std::io::Write;
                
                let mut file = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&target_path)?;
                file.write_all(args.content.as_bytes())?;
                
                let file_size = target_path.metadata()?.len();
                let brief = format!("追加 {} 字节", args.content.len());
                let output = format!(
                    "成功追加到文件: {}\n追加: {} 字节\n当前大小: {} 字节",
                    target_path.display(),
                    args.content.len(),
                    file_size
                );
                Ok(ToolResult::ok(brief, output))
            } else {
                // 覆盖模式
                fs::write(&target_path, &args.content)?;
                
                let brief = format!("写入 {} 字节", args.content.len());
                let output = format!(
                    "成功写入文件: {}\n大小: {} 字节",
                    target_path.display(),
                    args.content.len()
                );
                Ok(ToolResult::ok(brief, output))
            }
        }
        "file_replace" => {
            let args: FileReplaceArgs = serde_json::from_str(arguments)?;
            
            let target_path = {
                let p = Path::new(&args.path);
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    working_dir.join(p)
                }
            };
            
            // 验证文件存在
            if !target_path.exists() {
                return Ok(ToolResult::error(format!("文件不存在: {}", target_path.display())));
            }
            
            if !target_path.is_file() {
                return Ok(ToolResult::error(format!("不是文件: {}", target_path.display())));
            }
            
            // 需要审批（file_replace 也是危险操作）
            if require_approval && !is_action_approved("file_replace") {
                
                // 生成预览内容
                let preview = args.edits.iter()
                    .take(3)  // 最多显示 3 个编辑
                    .map(|e| {
                        let old_preview = if e.old.chars().count() > 40 {
                            let truncated: String = e.old.chars().take(40).collect();
                            format!("{}...", truncated)
                        } else {
                            e.old.clone()
                        };
                        let new_preview = if e.new.chars().count() > 40 {
                            let truncated: String = e.new.chars().take(40).collect();
                            format!("{}...", truncated)
                        } else {
                            e.new.clone()
                        };
                        format!("- Replace: {}\n  With: {}", old_preview, new_preview)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                
                let full_preview = if args.edits.len() > 3 {
                    format!("{}\n... and {} more edits", preview, args.edits.len() - 3)
                } else {
                    preview
                };
                
                let (approved, always, view_details) = prompt_approval(
                    "ReplaceFile",
                    &target_path.display().to_string(),
                    Some(&full_preview)
                )?;
                
                // 如果用户选择查看详细信息
                if view_details {
                    // 读取文件内容
                    let file_content = fs::read_to_string(&target_path)?;
                    
                    // 构建详细的编辑信息
                    let mut detailed_changes = String::new();
                    detailed_changes.push_str("\n=== 当前文件内容 ===\n");
                    detailed_changes.push_str(&file_content);
                    detailed_changes.push_str("\n\n=== 计划进行的更改 ===\n");
                    
                    for (i, edit) in args.edits.iter().enumerate() {
                        detailed_changes.push_str(&format!("\n编辑 #{}:\n", i + 1));
                        
                        if edit.replace_all {
                            detailed_changes.push_str(&format!("  替换所有出现的: '{}'", edit.old));
                        } else {
                            detailed_changes.push_str(&format!("  替换第一次出现的: '{}'", edit.old));
                        }
                        
                        if edit.new.contains('\n') || edit.new.chars().count() > 50 {
                            detailed_changes.push_str("\n  替换为 (多行):\n");
                            for line in edit.new.lines() {
                                detailed_changes.push_str(&format!("    {}\n", line));
                            }
                        } else {
                            detailed_changes.push_str(&format!("\n  替换为: '{}'", edit.new));
                        }
                    }
                    
                    let continue_operation = crate::ui::show_detailed_content(
                        "ReplaceFile",
                        &target_path.display().to_string(),
                        &detailed_changes
                    )?;
                    
                    if !continue_operation {
                        return Ok(ToolResult::error("用户取消了该操作".to_string()));
                    }
                }
                
                if !approved {
                    return Ok(ToolResult::error("用户拒绝了该操作".to_string()));
                }
                
                if always {
                    approve_action_for_session("file_replace");
                }
            }
            
            // 读取文件
            let mut content = fs::read_to_string(&target_path)?;
            
            // 检测厚实换行符风格
            let uses_crlf = content.contains("\r\n");
            
            // 关键：规范化换行符为 Unix \n
            // 这样整个处理上都会应清同一種换行符，避免匹配失败
            content = content.replace("\r\n", "\n");
            let original_content = content.clone();
            
            // 应用所有编辑
            let mut replacements_made = 0;
            let mut failed_edits = Vec::new();
            
            // 辅助函数：规范化字符串（为宽松匹配优化）
            fn normalize_whitespace(s: &str) -> String {
                s.lines()
                    .map(|line| line.trim())
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            
            for (edit_idx, edit) in args.edits.iter().enumerate() {
                if edit.regex {
                    // 正则表达式匹配
                    match Regex::new(&edit.old) {
                        Ok(re) => {
                            let count = re.find_iter(&content).count();
                            if count > 0 {
                                content = if edit.replace_all {
                                    replacements_made += count;
                                    re.replace_all(&content, &edit.new).into_owned()
                                } else {
                                    replacements_made += 1;
                                    re.replace(&content, &edit.new).into_owned()
                                };
                            } else {
                                failed_edits.push((edit_idx, edit.old.clone()));
                            }
                        }
                        Err(e) => {
                            return Ok(ToolResult::error(
                                format!("编辑#{}的正则表达式无效: {}", edit_idx + 1, e)
                            ));
                        }
                    }
                } else {
                    // 字符串匹配
                    let search_pattern = if edit.normalize {
                        normalize_whitespace(&edit.old)
                    } else {
                        edit.old.clone()
                    };
                    
                    content = if edit.replace_all {
                        let count = if edit.normalize {
                            let normalized_content = normalize_whitespace(&content);
                            normalized_content.matches(&search_pattern).count()
                        } else {
                            content.matches(&search_pattern).count()
                        };
                        replacements_made += count;
                        
                        if edit.normalize {
                            let normalized_content = normalize_whitespace(&content);
                            let normalized_result = normalized_content.replace(&search_pattern, &edit.new);
                            // 还原厚实的换行符
                            normalized_result.replace("\n", "\r\n")
                        } else {
                            content.replace(&search_pattern, &edit.new)
                        }
                    } else {
                        let found = if edit.normalize {
                            normalize_whitespace(&content).contains(&search_pattern)
                        } else {
                            content.contains(&search_pattern)
                        };
                        
                        if found {
                            replacements_made += 1;
                            if edit.normalize {
                                let normalized_content = normalize_whitespace(&content);
                                let normalized_result = normalized_content.replacen(&search_pattern, &edit.new, 1);
                                normalized_result.replace("\n", "\r\n")
                            } else {
                                content.replacen(&search_pattern, &edit.new, 1)
                            }
                        } else {
                            // 记录失败的编辑用于诊断
                            failed_edits.push((edit_idx, edit.old.clone()));
                            content
                        }
                    };
                }
            }
            
            // 检查是否有修改
            if content == original_content {
                // 生成详细的诊断信息
                let mut error_msg = String::from("未找到要替换的字符串。诊断信息：\n");
                
                for (idx, search_str) in failed_edits.iter() {
                    error_msg.push_str(&format!("\n编辑 #{}：\n", idx + 1));
                    error_msg.push_str(&format!("  搜索字符串长度: {} 字符\n", search_str.chars().count()));
                    error_msg.push_str(&format!("  搜索字符串 (前100字符): {}\n", 
                        if search_str.chars().count() > 100 {
                            search_str.chars().take(100).collect::<String>()
                        } else {
                            search_str.clone()
                        }
                    ));
                    error_msg.push_str(&format!("  包含换行符: {}\n", search_str.contains('\n')));
                    error_msg.push_str(&format!("  包含 \\r\\n: {}\n", search_str.contains("\r\n")));
                    
                    // 尝试找相似的内容作为建议
                    let mut suggestions = Vec::new();
                    for line in content.lines() {
                        if line.contains(&search_str.trim()) {
                            suggestions.push(line);
                        }
                    }
                    
                    if !suggestions.is_empty() && suggestions.len() <= 3 {
                        error_msg.push_str("  文件中发现相似内容（可能是空格/换行符差异）:\n");
                        for sugg in suggestions.iter().take(3) {
                            error_msg.push_str(&format!("    {}\n", sugg));
                        }
                    }
                }
                
                error_msg.push_str("\n提示：检查以下可能的问题:\n");
                error_msg.push_str("  1. 行结束符差异 (Windows \\r\\n vs Unix \\n)\n");
                error_msg.push_str("  2. 前后有额外空格\n");
                error_msg.push_str("  3. 缩进使用了不同的制表符或空格\n");
                error_msg.push_str("  4. 特殊字符编码差异\n");
                
                return Ok(ToolResult::error(error_msg));
            }
            
            // 写回文件
            // 根据厚实换行符风格恢复
            let final_content = if uses_crlf {
                content.replace("\n", "\r\n")
            } else {
                content  // Unix 风格，保持 \n
            };
            fs::write(&target_path, &final_content)?;
            
            let brief = format!("应用了 {} 个编辑，{} 个替换", args.edits.len(), replacements_made);
            let output = format!(
                "文件已更新: {}\n应用了 {} 个编辑\n共进行了 {} 个替换",
                target_path.display(),
                args.edits.len(),
                replacements_made
            );
            
            Ok(ToolResult::ok(brief, output))
        }
        "network_search_auto" => {
            let args: SearchArgs = serde_json::from_str(arguments)?;
            let max_results = limit_results(args.max_results);
            
            match crate::search_tool::search_auto(&args.keywords, max_results).await {
                Ok(results) => {
                    let brief = format!("找到 {} 个结果", results.len());
                    let mut output = format!("搜索关键词: {}\n找到 {} 个结果:\n\n", args.keywords, results.len());
                    
                    for (idx, result) in results.iter().enumerate() {
                        output.push_str(&format!(
                            "{}. [{}]\n   URL: {}\n   摘要: {}\n\n",
                            idx + 1,
                            result.title,
                            result.url,
                            result.snippet
                        ));
                    }
                    
                    Ok(ToolResult::ok(brief, output))
                }
                Err(e) => Ok(ToolResult::error(format!("搜索失败: {}", e)))
            }
        }
        "network_search_duckduckgo" => {
            let args: SearchArgs = serde_json::from_str(arguments)?;
            let max_results = limit_results(args.max_results);
            
            let client = crate::search_tool::SearchClient::new();
            match client.search_duckduckgo(&args.keywords, max_results).await {
                Ok(results) => {
                    let brief = format!("DuckDuckGo: 找到 {} 个结果", results.len());
                    let mut output = format!("搜索引擎: DuckDuckGo\n关键词: {}\n找到 {} 个结果:\n\n", args.keywords, results.len());
                    
                    for (idx, result) in results.iter().enumerate() {
                        output.push_str(&format!(
                            "{}. [{}]\n   URL: {}\n   摘要: {}\n\n",
                            idx + 1,
                            result.title,
                            result.url,
                            result.snippet
                        ));
                    }
                    
                    Ok(ToolResult::ok(brief, output))
                }
                Err(e) => Ok(ToolResult::error(format!("DuckDuckGo搜索失败: {}", e)))
            }
        }
        "network_search_bing" => {
            let args: SearchArgs = serde_json::from_str(arguments)?;
            let max_results = limit_results(args.max_results);
            
            let client = crate::search_tool::SearchClient::new();
            match client.search_bing(&args.keywords, max_results).await {
                Ok(results) => {
                    let brief = format!("Bing: 找到 {} 个结果", results.len());
                    let mut output = format!("搜索引擎: Bing\n关键词: {}\n找到 {} 个结果:\n\n", args.keywords, results.len());
                    
                    for (idx, result) in results.iter().enumerate() {
                        output.push_str(&format!(
                            "{}. [{}]\n   URL: {}\n   摘要: {}\n\n",
                            idx + 1,
                            result.title,
                            result.url,
                            result.snippet
                        ));
                    }
                    
                    Ok(ToolResult::ok(brief, output))
                }
                Err(e) => Ok(ToolResult::error(format!("Bing搜索失败: {}", e)))
            }
        }
        "file_diff_edit" => {
            let args: FileDiffEditArgs = serde_json::from_str(arguments)?;
            
            let target_path = {
                let p = Path::new(&args.path);
                if p.is_absolute() {
                    p.to_path_buf()
                } else {
                    working_dir.join(p)
                }
            };
            
            // 验证文件存在
            if !target_path.exists() {
                return Ok(ToolResult::error(format!("文件不存在: {}", target_path.display())));
            }
            
            if !target_path.is_file() {
                return Ok(ToolResult::error(format!("不是文件: {}", target_path.display())));
            }
            
            // 需要审批
            if require_approval && !is_action_approved("file_diff_edit") {
                let preview = args.hunks.iter()
                    .take(3)
                    .map(|h| format!("- Line {}: {} lines → {} chars", h.start_line, h.num_lines, h.new_content.chars().count()))
                    .collect::<Vec<_>>()
                    .join("\n");
                
                let full_preview = if args.hunks.len() > 3 {
                    format!("{}\n... and {} more hunks", preview, args.hunks.len() - 3)
                } else {
                    preview
                };
                
                let (approved, always, view_details) = prompt_approval(
                    "DiffEdit",
                    &target_path.display().to_string(),
                    Some(&full_preview)
                )?;
                
                if view_details {
                    let file_content = fs::read_to_string(&target_path)?;
                    let mut detailed_changes = String::new();
                    detailed_changes.push_str("\n=== 当前文件内容 ===\n");
                    detailed_changes.push_str(&file_content);
                    detailed_changes.push_str("\n\n=== 计划进行的更改 ===\n");
                    
                    for (i, hunk) in args.hunks.iter().enumerate() {
                        detailed_changes.push_str(&format!("\nHunk #{}\n", i + 1));
                        detailed_changes.push_str(&format!("  行号: {}\n", hunk.start_line));
                        detailed_changes.push_str(&format!("  原行数: {}\n", hunk.num_lines));
                        detailed_changes.push_str("  新内容:\n");
                        for line in hunk.new_content.lines() {
                            detailed_changes.push_str(&format!("    {}\n", line));
                        }
                    }
                    
                    let continue_operation = crate::ui::show_detailed_content(
                        "DiffEdit",
                        &target_path.display().to_string(),
                        &detailed_changes
                    )?;
                    
                    if !continue_operation {
                        return Ok(ToolResult::error("用户取消了该操作".to_string()));
                    }
                }
                
                if !approved {
                    return Ok(ToolResult::error("用户拒绝了该操作".to_string()));
                }
                
                if always {
                    approve_action_for_session("file_diff_edit");
                }
            }
            
            // 读取文件
            let content = fs::read_to_string(&target_path)?;
            let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
            
            // 检测换行符风格
            let uses_crlf = content.contains("\r\n");
            
            // 应用所有 hunk（从后到前，避免行号偏移）
            let mut hunks = args.hunks.clone();
            hunks.sort_by(|a, b| b.start_line.cmp(&a.start_line));
            
            // 记录所有修改的行范围，用于后续的验证输出
            let mut modified_ranges = Vec::new();
            
            for hunk in hunks.iter() {
                if hunk.start_line == 0 {
                    return Ok(ToolResult::error("行号必须从1开始".to_string()));
                }
                
                let start_idx = hunk.start_line - 1;
                let end_idx = std::cmp::min(start_idx + hunk.num_lines, lines.len());
                
                if start_idx > lines.len() {
                    return Ok(ToolResult::error(format!("行号超出范围: {}，文件总行数: {}", hunk.start_line, lines.len())));
                }
                
                // 记录修改范围（用于后续提取上下文）
                modified_ranges.push((start_idx, start_idx + hunk.new_content.lines().count()));
                
                // 构建新的行列表
                let mut new_lines = Vec::new();
                new_lines.extend_from_slice(&lines[..start_idx]);
                new_lines.extend(hunk.new_content.lines().map(|s| s.to_string()));
                new_lines.extend_from_slice(&lines[end_idx..]);
                
                lines = new_lines;
            }
            
            // 重建文件内容
            let new_content = lines.join("\n");
            let final_content = if uses_crlf {
                new_content.replace("\n", "\r\n")
            } else {
                new_content
            };
            
            fs::write(&target_path, &final_content)?;
            
            // **核心：直接从文件读取实际内容，并提取 ±3 行上下文**
            let actual_content = fs::read_to_string(&target_path)?;
            let actual_lines: Vec<&str> = actual_content.lines().collect();
            
            // 生成 diff_merge_result：合并所有修改范围的上下文
            let mut diff_merge_result = String::new();
            diff_merge_result.push_str("==== DIFF MERGE RESULT (from actual file) ====\n\n");
            
            // 合并所有修改范围，避免重复
            let mut all_context_ranges = Vec::new();
            for (mod_start, mod_end) in modified_ranges.iter() {
                let context_start = if *mod_start >= 3 { *mod_start - 3 } else { 0 };
                let context_end = std::cmp::min(*mod_end + 3, actual_lines.len());
                all_context_ranges.push((context_start, context_end));
            }
            
            // 合并重叠的范围
            all_context_ranges.sort();
            let mut merged_ranges = Vec::new();
            for (start, end) in all_context_ranges {
                if let Some((_last_start, last_end)) = merged_ranges.last_mut() {
                    if start <= *last_end {
                        *last_end = std::cmp::max(*last_end, end);
                    } else {
                        merged_ranges.push((start, end));
                    }
                } else {
                    merged_ranges.push((start, end));
                }
            }
            
            // 生成合并后的上下文输出
            for (range_start, range_end) in merged_ranges {
                for line_idx in range_start..range_end {
                    if line_idx < actual_lines.len() {
                        diff_merge_result.push_str(&format!("Line {:4}: {}\n", line_idx + 1, actual_lines[line_idx]));
                    }
                }
                diff_merge_result.push_str("\n");
            }
            
            let brief = format!("应用了 {} 个 hunk", args.hunks.len());
            let output = format!(
                "文件已更新: {}\n应用了 {} 个 diff hunk\n\n{}",
                target_path.display(),
                args.hunks.len(),
                diff_merge_result
            );
            
            let verification_prompt = "Please verify the DIFF merge result above. Check if all modifications are correct and there are no syntax errors (e.g., unclosed brackets, misaligned indentation). If everything looks good, you may continue. If there are any issues, describe the problem clearly.";
            
            Ok(ToolResult {
                success: true,
                brief,
                message: output,
                verification_required: true,
                verification_message: Some(verification_prompt.to_string()),
            })
        }
        "run_command" => {
            let args: RunCommandArgs = serde_json::from_str(arguments)?;
            
            // 加载命令配置
            let mut config = crate::tools::command_manager::CommandConfig::load()?;
            
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
                // 后台执行
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
            } else {
                // 前台执行
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
                        let _output_text = format!("Command: {}\nError: {}", args.command, e);
                        
                        Ok(ToolResult::error(brief))
                    }
                }
            }
        }
        _ => Ok(ToolResult::error(format!("未知工具: {}", name))),
    }
}
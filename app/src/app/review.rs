use anyhow::Result;
use api::ApiClient;
use chat;
use config::Config;
use history::{ChatSession, Message};
use i18n::I18n;
use serde::Deserialize;
use serde_json::Value;
use std::env;
use std::io;
use std::sync::mpsc;
use std::thread;
use tokio::runtime::Handle;
use ui::{self, ReviewRequest, Spinner};
use colored::Colorize;
use tools::{HookType, execute_hook, HookContext};

const MAX_PREVIEW_CHARS: usize = 4000;

pub fn install_review_handler(api_client: ApiClient, config: Config) {
    ui::set_review_handler(move |request: &ReviewRequest| {
        // Pre-Approval Hook
        if let Ok(cwd) = env::current_dir() {
             let hook_ctx = HookContext::new(cwd)
                 .with_env("FRIENDEV_ACTION", request.action)
                 .with_env("FRIENDEV_SUBJECT", request.subject);
             if let Err(e) = execute_hook(HookType::PreApproval, &hook_ctx) {
                 eprintln!("\n\x1b[33m[!] PreApproval Hook Error: {}\x1b[0m", e);
             }
        }

        // Try to load fresh config to respect runtime changes
        let (client, config_to_use) = match Config::load() {
            Ok(Some(loaded_config)) => {
                if let Some(sk_model) = &loaded_config.shorekeeper_model {
                    // Use shorekeeper specific model
                    let mut sk_cfg = loaded_config.clone();
                    sk_cfg.current_model = sk_model.clone();
                    (ApiClient::new(sk_cfg), loaded_config)
                } else {
                    // Use current main model (freshly loaded)
                    (ApiClient::new(loaded_config.clone()), loaded_config)
                }
            }
            _ => {
                // Fallback to initial state
                (api_client.clone(), config.clone())
            }
        };

        let handle = Handle::current();
        let owned_request = OwnedReviewRequest::from(request);

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let result =
                handle.block_on(async move { 
                    if owned_request.is_jury {
                        run_jury_review(&client, &config_to_use, &owned_request).await
                    } else {
                        run_review(&client, &config_to_use, &owned_request).await 
                    }
                });
            let _ = tx.send(result);
        });

        let result = match rx.recv() {
            Ok(Ok(approved)) => Ok(approved),
            Ok(Err(err)) => Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
            Err(recv_err) => Err(io::Error::new(io::ErrorKind::Other, recv_err.to_string())),
        };

        // Post-Approval Hook
        if let Ok(cwd) = env::current_dir() {
             let approved = match &result {
                 Ok(true) => "true",
                 _ => "false",
             };
             let hook_ctx = HookContext::new(cwd)
                 .with_env("FRIENDEV_ACTION", request.action)
                 .with_env("FRIENDEV_SUBJECT", request.subject)
                 .with_env("FRIENDEV_APPROVED", approved);
                 
             if let Err(e) = execute_hook(HookType::PostApproval, &hook_ctx) {
                 eprintln!("\n\x1b[33m[!] PostApproval Hook Error: {}\x1b[0m", e);
             }
        }

        result
    });
}

use futures::future::join_all;

async fn run_jury_review(
    client: &ApiClient,
    config: &Config,
    request: &OwnedReviewRequest,
) -> Result<bool> {
    let i18n = ui::get_i18n();

    println!(
        "\n  {} {}",
        "•",
        i18n.get("approval_jury_request")
            .replace("{}", &request.action)
    );

    let mut spinner = Spinner::new();
    spinner.render(&i18n.get("approval_jury_wait"));

    let working_dir = env::current_dir().unwrap_or_else(|_| env::temp_dir());
    let session = ChatSession::new(working_dir);

    let (preview, truncated) = format_preview(request.preview.as_deref(), &i18n);

    let system_prompt = "You are a member of Friendev's safety jury. Reply strictly as a minified JSON object with two keys: \"details\" (string describing the analysis in the same language as the user request) and \"approval\" (boolean, true if the action should proceed, false if it should be rejected). Do not output markdown, code fences, additional keys, or commentary. Never call tools.".to_string();

    let user_prompt = format!(
        "Evaluate whether the pending action should proceed.\nAction Type: {}\nTarget: {}\nContext Preview{}:\n{}\n\nBase your decision solely on this information. Prioritize security, data-loss, compliance, and stability risks. If information is insufficient, explain the uncertainty in \"details\" and set \"approval\" to false.",
        request.action,
        request.subject,
        if truncated { " (truncated)" } else { "" },
        preview
    );

    let message = Message {
        role: "user".to_string(),
        content: user_prompt.clone(),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    };

    let system_msg = Message {
        role: "system".to_string(),
        content: system_prompt.clone(),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    };

    // Spawn 3 parallel reviews
    let mut futures = Vec::new();
    for _ in 0..3 {
        let client = client.clone();
        let session = session.clone(); // ChatSession is lightweight to clone? Actually it has fields, but we need it for api call context.
        // Actually we can reuse session but api client handles chat history locally? 
        // No, api client sends messages vector. We just need to send same messages.
        
        let msgs = vec![system_msg.clone(), message.clone()];
        
        futures.push(tokio::spawn(async move {
            chat::send_and_receive(&client, msgs, &session).await
        }));
    }

    let results = join_all(futures).await;

    let mut votes_for = 0;
    let mut votes_against = 0;
    let mut details_list = Vec::new();

    for (idx, join_res) in results.into_iter().enumerate() {
        match join_res {
            Ok(Ok((response, _, _))) => {
                match parse_review_output(response.content.trim()) {
                    Ok(outcome) => {
                        if outcome.approval {
                            votes_for += 1;
                        } else {
                            votes_against += 1;
                        }
                        details_list.push((idx + 1, outcome));
                    }
                    Err(_) => {
                         // Parse error counts as abstain/fail? Or vote against for safety?
                         // Let's count as against.
                         votes_against += 1;
                         details_list.push((idx + 1, ReviewOutcome { approval: false, details: "Failed to parse response".to_string() }));
                    }
                }
            }
            Ok(Err(_)) | Err(_) => {
                votes_against += 1;
                details_list.push((idx + 1, ReviewOutcome { approval: false, details: "Request failed".to_string() }));
            }
        }
    }

    println!(
        "\r  {} {} ({}/{})                                          ",
        if votes_for >= 2 { "✓" } else { "✗" },
        i18n.get("approval_jury_done"),
        votes_for,
        votes_for + votes_against
    );

    println!();
    
    let passed = votes_for >= 2;

    println!("{}", i18n.get("approval_jury_result"));
    println!(
        "  {} {} ({})",
        i18n.get("approval_review_decision"),
        if passed {
            i18n.get("approval_review_decision_yes")
        } else {
            i18n.get("approval_review_decision_no")
        },
        format!("{}/3", votes_for)
    );

    println!("  {}", i18n.get("approval_jury_details"));

    for (juror, outcome) in details_list {
        let icon = if outcome.approval { "✓".green() } else { "✗".red() };
        println!("    {}: Juror #{}", icon, juror);
        for line in outcome.details.trim().lines() {
             if !line.trim().is_empty() {
                 println!("       {}", line.trim());
             }
        }
    }
    println!();

    Ok(passed)
}

async fn run_review(
    client: &ApiClient,
    _config: &Config,
    request: &OwnedReviewRequest,
) -> Result<bool> {
    let i18n = ui::get_i18n();

    println!(
        "\n  {} {}",
        "•",
        i18n.get("approval_review_request")
            .replace("{}", &request.action)
    );

    let mut spinner = Spinner::new();
    spinner.render(&i18n.get("approval_review_wait"));

    let working_dir = env::current_dir().unwrap_or_else(|_| env::temp_dir());
    let session = ChatSession::new(working_dir);

    let (preview, truncated) = format_preview(request.preview.as_deref(), &i18n);

    let system_prompt = "You are Friendev's safety review assistant. Reply strictly as a minified JSON object with two keys: \"details\" (string describing the analysis in the same language as the user request) and \"approval\" (boolean, true if the action should proceed, false if it should be rejected). Do not output markdown, code fences, additional keys, or commentary. Never call tools.".to_string();

    let user_prompt = format!(
        "Evaluate whether the pending action should proceed.\nAction Type: {}\nTarget: {}\nContext Preview{}:\n{}\n\nBase your decision solely on this information. Prioritize security, data-loss, compliance, and stability risks. If information is insufficient, explain the uncertainty in \"details\" and set \"approval\" to false.",
        request.action,
        request.subject,
        if truncated { " (truncated)" } else { "" },
        preview
    );

    let mut messages = Vec::with_capacity(2);
    messages.push(Message {
        role: "system".to_string(),
        content: system_prompt,
        tool_calls: None,
        tool_call_id: None,
        name: None,
    });
    messages.push(Message {
        role: "user".to_string(),
        content: user_prompt,
        tool_calls: None,
        tool_call_id: None,
        name: None,
    });

    let (response, tool_calls, _) = chat::send_and_receive(client, messages, &session).await?;

    if tool_calls.is_some() {
        anyhow::bail!(i18n.get("approval_review_tool_error"));
    }

    println!(
        "\r  {} {}                                                  ",
        "✓",
        i18n.get("approval_review_done")
    );

    println!();

    let raw_output = response.content.trim();
    match parse_review_output(raw_output) {
        Ok(outcome) => {
            println!("{}", i18n.get("approval_review_result"));
            println!(
                "  {} {}",
                i18n.get("approval_review_decision"),
                if outcome.approval {
                    i18n.get("approval_review_decision_yes")
                } else {
                    i18n.get("approval_review_decision_no")
                }
            );
            println!("  {}", i18n.get("approval_review_details"));

            for line in outcome.details.trim().lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                println!("    - {}", trimmed);
            }

            println!();
            Ok(outcome.approval)
        }
        Err(err) => {
            println!(
                "  [!] {}",
                i18n.get("approval_review_parse_error").replace("{}", &err)
            );
            println!("  {} {}", i18n.get("approval_review_raw"), raw_output);
            // On parse error, safe default is false? Or error?
            // Let's return error to trigger fallback or rejection
            anyhow::bail!("Failed to parse review output")
        }
    }
}

fn format_preview(preview: Option<&str>, i18n: &I18n) -> (String, bool) {
    match preview {
        Some(text) if !text.trim().is_empty() => {
            let (truncated, is_truncated) = truncate(text.trim(), MAX_PREVIEW_CHARS);
            (truncated, is_truncated)
        }
        _ => (i18n.get("approval_review_no_preview"), false),
    }
}

fn truncate(text: &str, max_chars: usize) -> (String, bool) {
    if text.chars().count() <= max_chars {
        return (text.to_string(), false);
    }

    let truncated: String = text.chars().take(max_chars).collect();
    (format!("{}...", truncated), true)
}

#[derive(Debug, Deserialize)]
struct ReviewOutcome {
    details: String,
    approval: bool,
}

#[derive(Clone)]
struct OwnedReviewRequest {
    action: String,
    subject: String,
    preview: Option<String>,
    is_jury: bool,
}

impl OwnedReviewRequest {
    fn from(request: &ReviewRequest<'_>) -> Self {
        Self {
            action: request.action.to_string(),
            subject: request.subject.to_string(),
            preview: request.preview.map(|s| s.to_string()),
            is_jury: request.is_jury,
        }
    }
}

fn parse_review_output(raw: &str) -> Result<ReviewOutcome, String> {
    if raw.is_empty() {
        return Err("empty output".to_string());
    }

    match serde_json::from_str::<ReviewOutcome>(raw) {
        Ok(outcome) => Ok(outcome),
        Err(primary_err) => {
            if let Ok(value) = serde_json::from_str::<Value>(raw) {
                if let Ok(outcome) = serde_json::from_value::<ReviewOutcome>(value) {
                    return Ok(outcome);
                }
            }
            Err(primary_err.to_string())
        }
    }
}

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
const MAX_PREVIEW_CHARS: usize = 4000;

pub fn install_review_handler(api_client: ApiClient, config: Config) {
    ui::set_review_handler(move |request: &ReviewRequest| {
        let client = api_client.clone();
        let cfg = config.clone();
        let handle = Handle::current();
        let owned_request = OwnedReviewRequest::from(request);

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let result =
                handle.block_on(async move { run_review(&client, &cfg, &owned_request).await });
            let _ = tx.send(result);
        });

        match rx.recv() {
            Ok(Ok(())) => Ok(()),
            Ok(Err(err)) => Err(io::Error::new(io::ErrorKind::Other, err.to_string())),
            Err(recv_err) => Err(io::Error::new(io::ErrorKind::Other, recv_err.to_string())),
        }
    });
}

async fn run_review(
    client: &ApiClient,
    config: &Config,
    request: &OwnedReviewRequest,
) -> Result<()> {
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

    let system_prompt = format!(
        "You are Friendev's safety review assistant. Reply strictly as a minified JSON object with two keys: \"details\" (string in {}) and \"decision\" (string, exactly \"同意\" or \"拒绝\"). Do not output markdown, code fences, additional keys, or commentary. Never call tools.",
        config.ai_language
    );

    let user_prompt = format!(
        "Evaluate whether the pending action should proceed.\nAction Type: {}\nTarget: {}\nContext Preview{}:\n{}\n\nBase your decision solely on this information. Prioritize security, data-loss, compliance, and stability risks. If information is insufficient, explain the uncertainty in \"details\" and choose \"拒绝\".",
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
            if outcome.decision != "同意" && outcome.decision != "拒绝" {
                println!(
                    "  [!] {}",
                    i18n.get("approval_review_bad_decision")
                        .replace("{}", &outcome.decision)
                );
            }

            println!("{}", i18n.get("approval_review_result"));
            println!(
                "  {} {}",
                i18n.get("approval_review_decision"),
                outcome.decision
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
        }
        Err(err) => {
            println!(
                "  [!] {}",
                i18n.get("approval_review_parse_error").replace("{}", &err)
            );
            println!("  {} {}", i18n.get("approval_review_raw"), raw_output);
        }
    }

    Ok(())
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
    decision: String,
}

struct OwnedReviewRequest {
    action: String,
    subject: String,
    preview: Option<String>,
}

impl OwnedReviewRequest {
    fn from(request: &ReviewRequest<'_>) -> Self {
        Self {
            action: request.action.to_string(),
            subject: request.subject.to_string(),
            preview: request.preview.map(|s| s.to_string()),
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
            // Try to locate JSON inside string in case of preamble
            if let Ok(value) = serde_json::from_str::<Value>(raw) {
                if let Ok(outcome) = serde_json::from_value::<ReviewOutcome>(value) {
                    return Ok(outcome);
                }
            }
            Err(primary_err.to_string())
        }
    }
}

use std::time::Duration;

use anyhow::{Context, Result};
use futures_util::StreamExt;
use html_escape::decode_html_entities;
use regex::Regex;
use reqwest::{header::CONTENT_TYPE, Client};
use tokio::time::timeout;
use ui::get_i18n;

use crate::tools::{args::FetchUrlArgs, utils::format_size};
use crate::tools::types::ToolResult;

const DEFAULT_MAX_BYTES: usize = 512 * 1024; // 512 KB
const MAX_ALLOWED_BYTES: usize = 1024 * 1024; // 1 MB
const MIN_ALLOWED_BYTES: usize = 1024; // 1 KB
const REQUEST_TIMEOUT: Duration = Duration::from_secs(15);

pub async fn execute_fetch_content(arguments: &str) -> Result<ToolResult> {
    let args: FetchUrlArgs = serde_json::from_str(arguments)?;
    let i18n = get_i18n();

    let parsed_url = match url::Url::parse(&args.url) {
        Ok(url) => url,
        Err(_) => {
            let tmpl = i18n.get("network_fetch_invalid_url");
            return Ok(ToolResult::error(tmpl.replace("{}", &args.url)));
        }
    };

    match parsed_url.scheme() {
        "http" | "https" => {}
        other => {
            let tmpl = i18n.get("network_fetch_unsupported_scheme");
            return Ok(ToolResult::error(tmpl.replace("{}", other)));
        }
    }

    let max_bytes = args
        .max_bytes
        .unwrap_or(DEFAULT_MAX_BYTES)
        .clamp(MIN_ALLOWED_BYTES, MAX_ALLOWED_BYTES);

    let client = Client::builder()
        .user_agent("FriendevTools/0.1 (+https://github.com/DerexTech/friendev)")
        .timeout(REQUEST_TIMEOUT)
        .redirect(reqwest::redirect::Policy::limited(5))
        .build()
        .context("client build failed")?;

    let response = match timeout(REQUEST_TIMEOUT, client.get(parsed_url.clone()).send()).await {
        Ok(result) => match result {
            Ok(resp) => resp,
            Err(error) => {
                let tmpl = i18n.get("network_fetch_request_error");
                return Ok(ToolResult::error(tmpl.replace("{}", &error.to_string())));
            }
        },
        Err(_) => {
            let tmpl = i18n.get("network_fetch_timeout");
            return Ok(ToolResult::error(tmpl));
        }
    };

    let status = response.status();

    if !status.is_success() {
        let reason = status.canonical_reason().unwrap_or("Unknown");
        let tmpl = i18n.get("network_fetch_status_error");
        return Ok(ToolResult::error(
            tmpl.replacen("{}", status.as_str(), 1)
                .replacen("{}", reason, 1),
        ));
    }

    let reported_length = response.content_length();

    if let Some(len) = reported_length {
        if len > max_bytes as u64 {
            let tmpl = i18n.get("network_fetch_too_large");
            return Ok(ToolResult::error(
                tmpl.replace("{}", &format_size(max_bytes as u64)),
            ));
        }
    }

    let content_type = response
        .headers()
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("")
        .to_string();

    if !is_textual_content(&content_type) {
        let tmpl = i18n.get("network_fetch_non_text");
        return Ok(ToolResult::error(tmpl.replace("{}", &content_type)));
    }

    let status_line = status.as_str().to_string();

    let mut stream = response.bytes_stream();
    let mut collected = Vec::new();
    let mut truncated = false;

    while let Some(chunk_result) = stream.next().await {
        let chunk = match chunk_result {
            Ok(data) => data,
            Err(error) => {
                let tmpl = i18n.get("network_fetch_request_error");
                return Ok(ToolResult::error(tmpl.replace("{}", &error.to_string())));
            }
        };

        if collected.len() >= max_bytes {
            truncated = true;
            break;
        }

        let remaining = max_bytes - collected.len();
        if chunk.len() <= remaining {
            collected.extend_from_slice(&chunk);
        } else {
            collected.extend_from_slice(&chunk[..remaining]);
            truncated = true;
            break;
        }
    }

    let mut notes = Vec::new();

    let raw_content = String::from_utf8_lossy(&collected).to_string();
    let is_html = is_html_content_type(&content_type) || is_probably_html(&raw_content);
    let content = if is_html {
        let cleaned = clean_html(&raw_content);
        notes.push(i18n.get("network_fetch_html_note"));
        cleaned
    } else {
        raw_content
    };

    let size_source = reported_length
        .map(|len| len as usize)
        .unwrap_or(collected.len());
    let size_label = format_size(size_source as u64);
    let brief_key = if truncated {
        "network_fetch_brief_truncated"
    } else {
        "network_fetch_brief"
    };
    let brief_tmpl = i18n.get(brief_key);
    let brief = brief_tmpl.replace("{}", &size_label);

    if truncated {
        let tmpl = i18n.get("network_fetch_truncated_note");
        notes.push(tmpl.replace("{}", &format_size(max_bytes as u64)));
    }

    let note = if notes.is_empty() {
        String::new()
    } else {
        notes.join("\n")
    };

    let output_tmpl = i18n.get("network_fetch_output");
    let output = output_tmpl
        .replacen("{}", parsed_url.as_str(), 1)
        .replacen("{}", &status_line, 1)
        .replacen("{}", &content_type, 1)
        .replacen("{}", &size_label, 1)
        .replacen("{}", &note, 1)
        .replacen("{}", &content, 1);

    Ok(ToolResult::ok(brief, output))
}

fn is_textual_content(content_type: &str) -> bool {
    if content_type.is_empty() {
        return true;
    }

    let lower = content_type.to_ascii_lowercase();
    lower.starts_with("text/")
        || lower.contains("json")
        || lower.contains("xml")
        || lower.contains("javascript")
        || lower.contains("+text")
}

fn is_html_content_type(content_type: &str) -> bool {
    content_type.to_ascii_lowercase().contains("html")
}

fn is_probably_html(content: &str) -> bool {
    let trimmed = content.trim_start();
    trimmed.starts_with("<!DOCTYPE html")
        || trimmed.starts_with("<html")
        || trimmed.starts_with("<head")
        || trimmed.starts_with("<body")
        || trimmed.contains("<div")
}

fn clean_html(html: &str) -> String {
    // Remove script and style blocks first
    let script_re = Regex::new(r"(?is)<script[^>]*>.*?</script>").unwrap();
    let style_re = Regex::new(r"(?is)<style[^>]*>.*?</style>").unwrap();

    let without_scripts = script_re.replace_all(html, " ");
    let without_code = style_re.replace_all(&without_scripts, " ");

    let decoded = decode_html_entities(&without_code);
    let without_tags = Regex::new(r"(?is)<[^>]+>")
        .unwrap()
        .replace_all(&decoded, " ");

    without_tags
        .split_whitespace()
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use httpmock::{Method::GET, MockServer};

    #[tokio::test]
    async fn test_fetch_content_success() {
        let server = MockServer::start_async().await;
        let mock = server.mock(|when, then| {
            when.method(GET).path("/text");
            then.status(200)
                .header("Content-Type", "text/plain; charset=utf-8")
                .body("Hello Friendev");
        });

        let args = serde_json::json!({
            "url": format!("{}", server.url("/text"))
        })
        .to_string();

        let result = execute_fetch_content(&args).await.unwrap();
        assert!(result.success, "expected success, got: {}", result.message);
        assert!(result.message.contains("Hello Friendev"));

        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_content_rejects_binary() {
        let server = MockServer::start_async().await;
        let mock = server.mock(|when, then| {
            when.method(GET).path("/bin");
            then.status(200)
                .header("Content-Type", "application/octet-stream")
                .body(vec![0, 1, 2, 3]);
        });

        let args = serde_json::json!({
            "url": format!("{}", server.url("/bin"))
        })
        .to_string();

        let result = execute_fetch_content(&args).await.unwrap();
        assert!(!result.success, "expected rejection for binary content");

        mock.assert();
    }

    #[tokio::test]
    async fn test_fetch_content_html_cleaning() {
        let server = MockServer::start_async().await;
        let mock = server.mock(|when, then| {
            when.method(GET).path("/page");
            then.status(200)
                .header("Content-Type", "text/html; charset=utf-8")
                .body("<html><body><h1>Hello</h1><script>alert('x');</script><p>World</p></body></html>");
        });

        let args = serde_json::json!({
            "url": format!("{}", server.url("/page"))
        })
        .to_string();

        let result = execute_fetch_content(&args).await.unwrap();
        assert!(result.success, "expected success for HTML");
        assert!(result.message.contains("Hello World"));
        assert!(!result.message.contains("<html>"));

        mock.assert();
    }
}

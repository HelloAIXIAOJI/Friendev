use super::html_parser::clean_html;
use super::types::SearchResult;
use crate::ui::get_i18n;
use anyhow::{anyhow, Result};
use reqwest::Client;
use scraper::{Html, Selector};

/// Search using Bing
pub async fn search_bing(
    client: &Client,
    keywords: &str,
    max_results: usize,
) -> Result<Vec<SearchResult>> {
    let url = format!(
        "https://www.bing.com/search?q={}",
        urlencoding::encode(keywords)
    );

    let response = client.get(&url).send().await.map_err(|e| {
        let i18n = get_i18n();
        anyhow!("{}: {}", i18n.get("search_bing_request_failed"), e)
    })?;

    if !response.status().is_success() {
        let i18n = get_i18n();
        return Err(anyhow!(
            "{}: {}",
            i18n.get("search_bing_status_code"),
            response.status()
        ));
    }

    let body = response.text().await.map_err(|e| {
        let i18n = get_i18n();
        anyhow!("{}: {}", i18n.get("search_bing_read_failed"), e)
    })?;

    parse_bing_html(&body, max_results)
}

/// Parse Bing HTML response
fn parse_bing_html(html: &str, max_results: usize) -> Result<Vec<SearchResult>> {
    let document = Html::parse_document(html);
    let mut results = Vec::new();

    // Bing search result selectors: in li.b_algo
    let li_selector =
        Selector::parse("li.b_algo").unwrap_or_else(|_| Selector::parse("div.b_algoBx").unwrap());

    for element in document.select(&li_selector).take(max_results) {
        // Extract title and link
        let h2_selector = Selector::parse("h2 a").ok();
        let (title, url) = h2_selector
            .as_ref()
            .and_then(|sel| element.select(sel).next())
            .map(|el| {
                let title = el.inner_html();
                let url = el.value().attr("href").unwrap_or("").to_string();
                (title, url)
            })
            .unwrap_or_default();

        // Extract snippet
        let p_selector = Selector::parse("p").ok();
        let snippet = p_selector
            .as_ref()
            .and_then(|sel| element.select(sel).next())
            .map(|el| el.inner_html())
            .unwrap_or_default();

        if !title.is_empty() && !url.is_empty() {
            let clean_title = clean_html(&title);
            let clean_snippet = clean_html(&snippet);

            results.push(SearchResult {
                title: clean_title,
                url,
                snippet: clean_snippet,
            });
        }
    }

    if results.is_empty() {
        let i18n = get_i18n();
        return Err(anyhow!("{}", i18n.get("search_bing_no_results")));
    }

    Ok(results)
}

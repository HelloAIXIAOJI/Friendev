use super::html_parser::clean_html;
use super::types::SearchResult;
use crate::ui::get_i18n;
use anyhow::{anyhow, Result};
use reqwest::Client;
use scraper::{Html, Selector};

/// Search using DuckDuckGo
pub async fn search_duckduckgo(
    client: &Client,
    keywords: &str,
    max_results: usize,
) -> Result<Vec<SearchResult>> {
    let url = format!(
        "https://html.duckduckgo.com/html?q={}",
        urlencoding::encode(keywords)
    );

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| anyhow!("DDG Request failed: {}", e))?;

    if !response.status().is_success() {
        return Err(anyhow!("DDG Code: {}", response.status()));
    }

    let body = response
        .text()
        .await
        .map_err(|e| anyhow!("Failed to read response from DDG: {}", e))?;

    parse_duckduckgo_html(&body, max_results)
}

/// Parse DuckDuckGo HTML response
fn parse_duckduckgo_html(html: &str, max_results: usize) -> Result<Vec<SearchResult>> {
    let document = Html::parse_document(html);
    let mut results = Vec::new();

    // DuckDuckGo HTML version selectors: search results in div.result
    // or in div.links_main under div[class~="result"]
    let selector = match Selector::parse("div.results div[class~='result']") {
        Ok(s) => s,
        Err(_) => Selector::parse("div.results > article")
            .unwrap_or_else(|_| Selector::parse("div[class*='result']").unwrap()),
    };

    for element in document.select(&selector).take(max_results) {
        // Extract title and link
        let title_link_selector = Selector::parse("a.result__a").ok();
        let title_link = title_link_selector
            .as_ref()
            .and_then(|sel| element.select(sel).next())
            .and_then(|el| el.value().attr("href"));

        let title = title_link_selector
            .as_ref()
            .and_then(|sel| element.select(sel).next())
            .map(|el| el.inner_html())
            .unwrap_or_default();

        // Extract snippet
        let snippet_selector = Selector::parse("a.result__snippet").ok();
        let snippet = snippet_selector
            .as_ref()
            .and_then(|sel| element.select(sel).next())
            .map(|el| el.inner_html())
            .unwrap_or_default();

        if let Some(url) = title_link {
            let clean_title = clean_html(&title);
            let clean_snippet = clean_html(&snippet);

            if !clean_title.is_empty() && !url.is_empty() {
                results.push(SearchResult {
                    title: clean_title,
                    url: url.to_string(),
                    snippet: clean_snippet,
                });
            }
        }
    }

    if results.is_empty() {
        let i18n = get_i18n();
        return Err(anyhow!("{}", i18n.get("search_ddg_no_results")));
    }

    Ok(results)
}

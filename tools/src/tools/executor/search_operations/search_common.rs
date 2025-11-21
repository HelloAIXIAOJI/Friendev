use crate::tools::types::ToolResult;
use crate::ui::get_i18n;

/// Format search results into a readable output
pub fn format_search_results(
    keywords: &str,
    results: &[crate::search_tool::SearchResult],
    engine_name: Option<&str>,
) -> String {
    let i18n = get_i18n();

    let engine_prefix = if let Some(name) = engine_name {
        let tmpl = i18n.get("search_engine_prefix");
        tmpl.replace("{}", name)
    } else {
        String::new()
    };

    let mut output = format!(
        "{}{}: {}\n{}: {}\n\n",
        engine_prefix,
        i18n.get("search_keywords_label"),
        keywords,
        i18n.get("search_found_label"),
        results.len()
    );

    for (idx, result) in results.iter().enumerate() {
        output.push_str(&format!(
            "{}. [{}]\n   {}: {}\n   {}: {}\n\n",
            idx + 1,
            result.title,
            i18n.get("search_url_label"),
            result.url,
            i18n.get("search_snippet_label"),
            result.snippet
        ));
    }

    output
}

/// Generate brief description for search results
pub fn generate_brief(count: usize, engine_name: Option<&str>) -> String {
    let i18n = get_i18n();
    if let Some(name) = engine_name {
        let tmpl = i18n.get("search_brief_with_engine");
        tmpl.replacen("{}", name, 1)
            .replacen("{}", &count.to_string(), 1)
    } else {
        let tmpl = i18n.get("search_brief");
        tmpl.replace("{}", &count.to_string())
    }
}

/// Create a successful search result
pub fn create_search_result(
    keywords: &str,
    results: &[crate::search_tool::SearchResult],
    engine_name: Option<&str>,
) -> ToolResult {
    let brief = generate_brief(results.len(), engine_name);
    let output = format_search_results(keywords, results, engine_name);
    ToolResult::ok(brief, output)
}

/// Create an error result for search failure
pub fn create_search_error(error_msg: &str, engine_name: Option<&str>) -> ToolResult {
    let i18n = get_i18n();

    let error_text = if let Some(name) = engine_name {
        let tmpl = i18n.get("search_error_with_engine");
        tmpl.replacen("{}", name, 1).replacen("{}", error_msg, 1)
    } else {
        let tmpl = i18n.get("search_error");
        tmpl.replace("{}", error_msg)
    };
    ToolResult::error(error_text)
}

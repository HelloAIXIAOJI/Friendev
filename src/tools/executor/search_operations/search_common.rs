use crate::tools::types::ToolResult;

/// Format search results into a readable output
pub fn format_search_results(
    keywords: &str,
    results: &[crate::search_tool::SearchResult],
    engine_name: Option<&str>,
) -> String {
    let engine_prefix = if let Some(name) = engine_name {
        format!("搜索引擎: {}\n", name)
    } else {
        String::new()
    };

    let mut output = format!(
        "{}关键词: {}\n找到 {} 个结果:\n\n",
        engine_prefix,
        keywords,
        results.len()
    );

    for (idx, result) in results.iter().enumerate() {
        output.push_str(&format!(
            "{}. [{}]\n   URL: {}\n   摘要: {}\n\n",
            idx + 1,
            result.title,
            result.url,
            result.snippet
        ));
    }

    output
}

/// Generate brief description for search results
pub fn generate_brief(count: usize, engine_name: Option<&str>) -> String {
    if let Some(name) = engine_name {
        format!("{}: 找到 {} 个结果", name, count)
    } else {
        format!("找到 {} 个结果", count)
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
    let error_text = if let Some(name) = engine_name {
        format!("{}搜索失败: {}", name, error_msg)
    } else {
        format!("搜索失败: {}", error_msg)
    };
    ToolResult::error(error_text)
}

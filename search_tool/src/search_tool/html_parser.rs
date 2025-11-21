/// Clean HTML tags and HTML entities from text
pub fn clean_html(html: &str) -> String {
    // Decode HTML entities
    let text = html_escape::decode_html_entities(html);

    // Remove HTML tags
    let re_tags = regex::Regex::new(r"<[^>]+>").unwrap_or_else(|_| regex::Regex::new("").unwrap());
    let cleaned = re_tags.replace_all(&text, "");

    // Remove excess whitespace
    cleaned
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .trim()
        .to_string()
}

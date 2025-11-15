// 限制max_results到20以内
pub fn limit_results(max: usize) -> usize {
    std::cmp::min(std::cmp::max(1, max), 20)
}

// 辅助函数：规范化字符串（为宽松匹配优化）
pub fn normalize_whitespace(s: &str) -> String {
    s.lines()
        .map(|line| line.trim())
        .collect::<Vec<_>>()
        .join("\n")
}

/// 格式化文件大小显示
pub fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// 自动生成工具列表描述，用于系统提示词
pub fn get_tools_description() -> String {
    let tools = crate::tools::definitions::get_available_tools();
    let mut descriptions = Vec::new();

    for tool in tools {
        descriptions.push(format!(
            "- {}: {}",
            tool.function.name, tool.function.description
        ));
    }

    descriptions.join("\n")
}

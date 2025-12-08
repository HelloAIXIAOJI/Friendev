pub mod list;
pub mod info;
pub mod call;

pub fn parse_tool_spec(tool_spec: &str) -> (Option<&str>, &str) {
    if let Some(pos) = tool_spec.find('/') {
        (Some(&tool_spec[..pos]), &tool_spec[pos + 1..])
    } else {
        (None, tool_spec)
    }
}

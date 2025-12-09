mod list;
mod info;
mod use_prompt;

pub fn parse_prompt_spec(prompt_spec: &str) -> (Option<&str>, &str) {
    // 如果包含 / 且不是 :// 格式，按第一个 / 分割（server/prompt 格式）
    if !prompt_spec.contains("://") && prompt_spec.contains('/') {
        if let Some(pos) = prompt_spec.find('/') {
            (Some(&prompt_spec[..pos]), &prompt_spec[pos + 1..])
        } else {
            (None, prompt_spec)
        }
    } else {
        (None, prompt_spec)
    }
}

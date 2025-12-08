mod list;
mod read;
mod info;
mod download;

pub fn parse_resource_spec(resource_spec: &str) -> (Option<&str>, &str) {
    // 如果包含 :// 说明是 URI，不应该按 / 分割
    if resource_spec.contains("://") {
        (None, resource_spec)
    } else if let Some(pos) = resource_spec.find('/') {
        // 只有在不是 URI 的情况下才按 / 分割（server/uri 格式）
        (Some(&resource_spec[..pos]), &resource_spec[pos + 1..])
    } else {
        (None, resource_spec)
    }
}

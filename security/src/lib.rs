/// 检查用户输入是否包含可疑的控制标记
pub fn is_input_suspicious(input: &str) -> bool {
    // 检查 ChatML 格式标记
    if input.contains("<|im_start|>") && input.contains("<|im_end|>") {
        return true;
    }

    // 检查其他常见的特殊标记
    let suspicious_tokens = [
        "<|endoftext|>",
        "<|system|>",
        "<|user|>",
        "<|assistant|>",
        "</s>",
        "<s>",
    ];

    for token in &suspicious_tokens {
        if input.contains(token) {
            return true;
        }
    }

    false
}

mod approval_prompt;
mod spinner;
mod tool_call_display;

use crate::config::Config;
use crate::i18n::I18n;

// 重新导出主要的公共 API
pub use approval_prompt::{prompt_approval, show_detailed_content};
pub use spinner::Spinner;
pub use tool_call_display::{extract_key_argument, ToolCallDisplay};

/// 获取当前 UI 语言对应的 I18n 实例
pub fn get_i18n() -> I18n {
    let ui_lang = Config::load()
        .ok()
        .and_then(|c| c.map(|c| c.ui_language))
        .unwrap_or_else(|| "enus".to_string());
    I18n::new(&ui_lang)
}

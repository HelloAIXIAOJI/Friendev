use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Mutex;

/// 会话级审批状态
static APPROVED_ACTIONS: Mutex<Option<HashSet<String>>> = Mutex::new(None);

/// 检查操作是否已被批准
pub fn is_action_approved(action: &str) -> bool {
    let mut approved = APPROVED_ACTIONS.lock().unwrap();
    if approved.is_none() {
        *approved = Some(HashSet::new());
    }
    approved.as_ref().unwrap().contains(action)
}

/// 添加操作到已批准列表
pub fn approve_action_for_session(action: &str) {
    let mut approved = APPROVED_ACTIONS.lock().unwrap();
    if approved.is_none() {
        *approved = Some(HashSet::new());
    }
    approved.as_mut().unwrap().insert(action.to_string());
}

/// 工具执行结果
pub struct ToolResult {
    pub success: bool,
    pub brief: String,
    pub message: String,
    pub verification_required: bool,
    pub verification_message: Option<String>,
}

impl ToolResult {
    pub fn ok(brief: String, output: String) -> Self {
        Self {
            success: true,
            brief,
            message: output,
            verification_required: false,
            verification_message: None,
        }
    }

    pub fn error(brief: String) -> Self {
        Self {
            success: false,
            brief: brief.clone(),
            message: brief,
            verification_required: false,
            verification_message: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: ToolFunction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunction {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

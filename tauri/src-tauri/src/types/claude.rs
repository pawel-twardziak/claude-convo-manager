use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ClaudeHistoryEntry {
    pub display: Option<String>,
    pub timestamp: Option<f64>,
    pub project: Option<String>,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SessionsIndexFile {
    pub version: Option<i64>,
    pub entries: Option<Vec<SessionsIndexEntry>>,
}

#[derive(Debug, Deserialize)]
pub struct SessionsIndexEntry {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "projectPath")]
    pub project_path: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ActiveSessionFile {
    pub pid: Option<i64>,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubagentMeta {
    #[serde(rename = "agentType")]
    pub agent_type: Option<String>,
    pub description: Option<String>,
}

// Content block types
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ContentBlock {
    #[serde(rename = "thinking")]
    Thinking { thinking: String },
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(rename = "tool_use")]
    ToolUse {
        id: Option<String>,
        name: String,
        input: serde_json::Value,
    },
    #[serde(rename = "tool_result")]
    ToolResult {
        tool_use_id: Option<String>,
        content: Option<serde_json::Value>,
        is_error: Option<bool>,
    },
}

#[derive(Debug, Deserialize)]
pub struct TokenUsage {
    pub input_tokens: Option<i64>,
    pub output_tokens: Option<i64>,
    pub cache_creation_input_tokens: Option<i64>,
    pub cache_read_input_tokens: Option<i64>,
    pub inference_geo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AssistantMessageInner {
    pub model: Option<String>,
    pub id: Option<String>,
    pub content: Option<Vec<ContentBlock>>,
    pub stop_reason: Option<String>,
    pub usage: Option<TokenUsage>,
}

#[derive(Debug, Deserialize)]
pub struct UserMessageInner {
    pub role: Option<String>,
    pub content: Option<serde_json::Value>,
}

// We parse each JSONL line as a GenericMessage first,
// then match on the `type` field.
#[derive(Debug, Deserialize)]
pub struct GenericMessage {
    #[serde(rename = "type")]
    pub msg_type: String,

    // Common fields
    pub uuid: Option<String>,
    #[serde(rename = "parentUuid")]
    pub parent_uuid: Option<String>,
    #[serde(rename = "isSidechain")]
    pub is_sidechain: Option<bool>,
    pub timestamp: Option<String>,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    #[serde(rename = "agentId")]
    pub agent_id: Option<String>,
    #[serde(rename = "isMeta")]
    pub is_meta: Option<bool>,

    // User-specific fields
    #[serde(rename = "gitBranch")]
    pub git_branch: Option<String>,
    pub cwd: Option<String>,
    pub version: Option<String>,
    #[serde(rename = "permissionMode")]
    pub permission_mode: Option<String>,

    // Assistant-specific: nested message object
    pub message: Option<serde_json::Value>,

    // custom-title
    #[serde(rename = "customTitle")]
    pub custom_title: Option<String>,

    // agent-name
    #[serde(rename = "agentName")]
    pub agent_name: Option<String>,
}

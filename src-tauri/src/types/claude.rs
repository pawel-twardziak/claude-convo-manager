use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ClaudeHistoryEntry {
    pub display: Option<String>,
    pub timestamp: Option<f64>,
    pub project: Option<String>,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SessionsIndexFile {
    pub version: Option<i64>,
    pub entries: Option<Vec<SessionsIndexEntry>>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct SessionsIndexEntry {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "projectPath")]
    pub project_path: Option<String>,
    #[serde(rename = "fullPath")]
    pub full_path: Option<String>,
    #[serde(rename = "fileMtime")]
    pub file_mtime: Option<i64>,
    #[serde(rename = "firstPrompt")]
    pub first_prompt: Option<String>,
    pub summary: Option<String>,
    #[serde(rename = "messageCount")]
    pub message_count: Option<i64>,
    pub created: Option<String>,
    pub modified: Option<String>,
    #[serde(rename = "gitBranch")]
    pub git_branch: Option<String>,
    #[serde(rename = "isSidechain")]
    pub is_sidechain: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ActiveSessionFile {
    pub pid: Option<i64>,
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    pub status: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<i64>,
    pub kind: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SubagentMeta {
    #[serde(rename = "agentType")]
    pub agent_type: Option<String>,
    pub description: Option<String>,
    pub slug: Option<String>,
}

/// First-line shape of an `agent-*.jsonl` written by older Claude Code
/// (≤ 2.1.74), used as a fallback when no `.meta.json` sidecar exists.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SubagentJsonlHeader {
    #[serde(rename = "agentId")]
    pub agent_id: Option<String>,
    pub slug: Option<String>,
    pub version: Option<String>,
    pub cwd: Option<String>,
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

// We parse each JSONL line as a GenericMessage first,
// then match on the `type` field.
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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

    // ai-title
    #[serde(rename = "aiTitle")]
    pub ai_title: Option<String>,

    // user/tool-result linkage and interruption signal
    #[serde(rename = "sourceToolAssistantUUID")]
    pub source_tool_assistant_uuid: Option<String>,
    #[serde(rename = "toolUseResult")]
    pub tool_use_result: Option<serde_json::Value>,
}

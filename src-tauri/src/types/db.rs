use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionWithProject {
    pub id: String,
    pub project_id: i64,
    pub file_path: String,
    pub file_mtime: Option<f64>,
    pub file_size: Option<i64>,
    pub first_prompt: Option<String>,
    pub custom_title: Option<String>,
    pub message_count: i64,
    pub user_message_count: i64,
    pub assistant_message_count: i64,
    pub tool_use_count: i64,
    pub git_branch: Option<String>,
    pub cwd: Option<String>,
    pub model: Option<String>,
    pub version: Option<String>,
    pub permission_mode: Option<String>,
    pub is_sidechain: i64,
    pub is_active: i64,
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
    pub total_cache_creation_tokens: i64,
    pub total_cache_read_tokens: i64,
    pub estimated_cost_usd: f64,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
    pub synced_at: String,
    pub project_path: String,
    pub project_display_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageRow {
    pub id: i64,
    pub uuid: Option<String>,
    pub session_id: String,
    pub parent_uuid: Option<String>,
    #[serde(rename = "type")]
    pub msg_type: String,
    pub role: Option<String>,
    pub is_sidechain: i64,
    pub agent_id: Option<String>,
    pub model: Option<String>,
    pub content_text: Option<String>,
    pub content_json: Option<String>,
    pub has_tool_use: i64,
    pub has_thinking: i64,
    pub tool_names: Option<String>,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub stop_reason: Option<String>,
    pub timestamp: Option<String>,
    pub line_number: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SearchResult {
    pub message_id: i64,
    pub session_id: String,
    pub project_display_name: Option<String>,
    pub session_title: Option<String>,
    pub snippet: String,
    pub timestamp: Option<String>,
    pub rank: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DashboardStats {
    #[serde(rename = "totalSessions")]
    pub total_sessions: i64,
    #[serde(rename = "totalProjects")]
    pub total_projects: i64,
    #[serde(rename = "totalMessages")]
    pub total_messages: i64,
    #[serde(rename = "totalInputTokens")]
    pub total_input_tokens: i64,
    #[serde(rename = "totalOutputTokens")]
    pub total_output_tokens: i64,
    #[serde(rename = "totalCacheCreationTokens")]
    pub total_cache_creation_tokens: i64,
    #[serde(rename = "totalCacheReadTokens")]
    pub total_cache_read_tokens: i64,
    #[serde(rename = "estimatedTotalCost")]
    pub estimated_total_cost: f64,
    #[serde(rename = "activeSessions")]
    pub active_sessions: i64,
    #[serde(rename = "todaySessions")]
    pub today_sessions: i64,
    #[serde(rename = "avgMessageCount")]
    pub avg_message_count: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectWithStats {
    pub id: i64,
    #[serde(rename = "encodedName")]
    pub encoded_name: String,
    #[serde(rename = "projectPath")]
    pub project_path: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "sessionCount")]
    pub session_count: i64,
    #[serde(rename = "totalTokens")]
    pub total_tokens: i64,
    #[serde(rename = "lastActivityAt")]
    pub last_activity_at: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "totalInputTokens")]
    pub total_input_tokens: i64,
    #[serde(rename = "totalOutputTokens")]
    pub total_output_tokens: i64,
    #[serde(rename = "totalCacheCreationTokens")]
    pub total_cache_creation_tokens: i64,
    #[serde(rename = "totalCacheReadTokens")]
    pub total_cache_read_tokens: i64,
    #[serde(rename = "estimatedCostUsd")]
    pub estimated_cost_usd: f64,
    #[serde(rename = "distinctModels")]
    pub distinct_models: Option<String>,
    #[serde(rename = "distinctBranches")]
    pub distinct_branches: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterOptionProject {
    pub id: i64,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "sessionCount")]
    pub session_count: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FilterOptions {
    pub projects: Vec<FilterOptionProject>,
    pub branches: Vec<String>,
    pub models: Vec<String>,
    #[serde(rename = "dateRange")]
    pub date_range: DateRange,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DateRange {
    pub min: Option<String>,
    pub max: Option<String>,
}

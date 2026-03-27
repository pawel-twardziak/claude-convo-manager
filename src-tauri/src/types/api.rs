use serde::{Deserialize, Serialize};

use super::db::{MessageRow, ProjectWithStats, SearchResult, SessionWithProject};

#[derive(Debug, Deserialize)]
pub struct GetSessionsParams {
    #[serde(rename = "projectId")]
    pub project_id: Option<i64>,
    #[serde(rename = "gitBranch")]
    pub git_branch: Option<String>,
    pub model: Option<String>,
    #[serde(rename = "dateFrom")]
    pub date_from: Option<String>,
    #[serde(rename = "dateTo")]
    pub date_to: Option<String>,
    pub search: Option<String>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortDir")]
    pub sort_dir: Option<String>,
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct GetSessionsResponse {
    pub sessions: Vec<SessionWithProject>,
    pub total: i64,
    pub page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
}

#[derive(Debug, Deserialize)]
pub struct GetMessagesParams {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    #[serde(rename = "excludeSidechain")]
    pub exclude_sidechain: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct GetMessagesResponse {
    pub messages: Vec<MessageRow>,
    pub total: i64,
    #[serde(rename = "hasMore")]
    pub has_more: bool,
}

#[derive(Debug, Deserialize)]
pub struct SearchMessagesParams {
    pub query: String,
    #[serde(rename = "projectId")]
    pub project_id: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct SearchMessagesResponse {
    pub results: Vec<SearchResult>,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct GetTokenUsageParams {
    #[serde(rename = "groupBy")]
    pub group_by: Option<String>,
    #[serde(rename = "dateFrom")]
    pub date_from: Option<String>,
    #[serde(rename = "dateTo")]
    pub date_to: Option<String>,
    #[serde(rename = "projectId")]
    pub project_id: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct TokenUsageEntry {
    pub date: String,
    #[serde(rename = "inputTokens")]
    pub input_tokens: i64,
    #[serde(rename = "outputTokens")]
    pub output_tokens: i64,
    pub cost: f64,
    #[serde(rename = "sessionCount")]
    pub session_count: i64,
}

#[derive(Debug, Serialize)]
pub struct ProjectBreakdownEntry {
    pub name: Option<String>,
    pub sessions: i64,
    pub tokens: i64,
}

#[derive(Debug, Serialize)]
pub struct ActivityEntry {
    pub date: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct SyncResult {
    pub ok: bool,
    pub sessions: i64,
    pub messages: i64,
}

#[derive(Debug, Serialize, Clone)]
pub struct SyncProgress {
    pub phase: String,
    pub current: i64,
    pub total: i64,
}

#[derive(Debug, Deserialize)]
pub struct GetProjectsParams {
    pub search: Option<String>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortDir")]
    pub sort_dir: Option<String>,
    pub page: Option<i64>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct GetProjectsResponse {
    pub projects: Vec<ProjectWithStats>,
    pub total: i64,
    pub page: i64,
    #[serde(rename = "pageSize")]
    pub page_size: i64,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceAllParams {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "searchTerm")]
    pub search_term: String,
    #[serde(rename = "replaceTerm")]
    pub replace_term: String,
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ReplaceOneParams {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "searchTerm")]
    pub search_term: String,
    #[serde(rename = "replaceTerm")]
    pub replace_term: String,
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[serde(rename = "occurrenceIndex")]
    pub occurrence_index: usize,
}

#[derive(Debug, Serialize)]
pub struct ReplaceResult {
    #[serde(rename = "replacedCount")]
    pub replaced_count: usize,
}

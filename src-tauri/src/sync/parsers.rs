use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::types::claude::{AssistantMessageInner, ContentBlock, GenericMessage};

#[derive(Debug, Clone)]
pub struct ParsedMessage {
    pub uuid: Option<String>,
    pub parent_uuid: Option<String>,
    pub msg_type: String,
    pub role: Option<String>,
    pub is_sidechain: bool,
    pub agent_id: Option<String>,
    pub model: Option<String>,
    pub content_text: String,
    pub content_json: Option<String>,
    pub has_tool_use: bool,
    pub has_thinking: bool,
    pub tool_names: Vec<String>,
    pub input_tokens: i64,
    pub output_tokens: i64,
    pub stop_reason: Option<String>,
    pub timestamp: Option<String>,
    pub line_number: i64,
}

#[derive(Debug)]
pub struct SessionMetadata {
    pub first_prompt: Option<String>,
    pub custom_title: Option<String>,
    pub models: HashMap<String, i64>,
    pub total_input_tokens: i64,
    pub total_output_tokens: i64,
    pub total_cache_creation_tokens: i64,
    pub total_cache_read_tokens: i64,
    pub tool_use_count: i64,
    pub user_message_count: i64,
    pub assistant_message_count: i64,
    pub git_branch: Option<String>,
    pub cwd: Option<String>,
    pub version: Option<String>,
    pub permission_mode: Option<String>,
    pub created_at: Option<String>,
    pub modified_at: Option<String>,
}

pub struct SessionParseResult {
    pub messages: Vec<ParsedMessage>,
    pub metadata: SessionMetadata,
}

fn extract_content_text_from_value(content: &serde_json::Value) -> String {
    if let Some(s) = content.as_str() {
        return s.to_string();
    }
    if let Some(arr) = content.as_array() {
        let mut parts = Vec::new();
        for block in arr {
            let block_type = block.get("type").and_then(|t| t.as_str()).unwrap_or("");
            match block_type {
                "text" => {
                    if let Some(text) = block.get("text").and_then(|t| t.as_str()) {
                        parts.push(text.to_string());
                    }
                }
                "tool_result" => {
                    let is_error = block
                        .get("is_error")
                        .and_then(|e| e.as_bool())
                        .unwrap_or(false);
                    if let Some(c) = block.get("content").and_then(|c| c.as_str()) {
                        if is_error {
                            parts.push(format!("[Error: {}]", &c[..c.len().min(200)]));
                        } else if c.len() < 1000 {
                            parts.push(c.to_string());
                        }
                    }
                }
                "tool_use" => {
                    if let Some(name) = block.get("name").and_then(|n| n.as_str()) {
                        parts.push(format!("[Tool: {}]", name));
                    }
                }
                _ => {}
            }
        }
        return parts.join("\n");
    }
    String::new()
}

fn extract_assistant_info(
    content: &[ContentBlock],
) -> (String, bool, bool, Vec<String>) {
    let mut text_parts = Vec::new();
    let mut tool_names = Vec::new();
    let mut has_tool_use = false;
    let mut has_thinking = false;

    for block in content {
        match block {
            ContentBlock::Text { text } => {
                text_parts.push(text.clone());
            }
            ContentBlock::ToolUse { name, .. } => {
                has_tool_use = true;
                tool_names.push(name.clone());
            }
            ContentBlock::Thinking { .. } => {
                has_thinking = true;
            }
            ContentBlock::ToolResult { .. } => {}
        }
    }

    (text_parts.join("\n"), has_tool_use, has_thinking, tool_names)
}

fn is_streaming_partial(inner: &AssistantMessageInner) -> bool {
    inner.stop_reason.is_none()
        && inner
            .usage
            .as_ref()
            .and_then(|u| u.inference_geo.as_deref())
            == Some("not_available")
}

pub fn parse_session_file(file_path: &Path) -> Result<SessionParseResult, String> {
    let file =
        File::open(file_path).map_err(|e| format!("Failed to open {}: {}", file_path.display(), e))?;
    let reader = BufReader::new(file);

    let mut messages: Vec<ParsedMessage> = Vec::new();
    let mut metadata = SessionMetadata {
        first_prompt: None,
        custom_title: None,
        models: HashMap::new(),
        total_input_tokens: 0,
        total_output_tokens: 0,
        total_cache_creation_tokens: 0,
        total_cache_read_tokens: 0,
        tool_use_count: 0,
        user_message_count: 0,
        assistant_message_count: 0,
        git_branch: None,
        cwd: None,
        version: None,
        permission_mode: None,
        created_at: None,
        modified_at: None,
    };

    let mut assistant_message_ids: HashMap<String, usize> = HashMap::new();
    let mut line_number: i64 = 0;

    for line_result in reader.lines() {
        line_number += 1;
        let line = match line_result {
            Ok(l) => l,
            Err(_) => continue,
        };
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let parsed: GenericMessage = match serde_json::from_str(trimmed) {
            Ok(m) => m,
            Err(_) => continue,
        };

        let msg_type = parsed.msg_type.as_str();

        // Skip types we don't store
        if matches!(
            msg_type,
            "progress" | "file-history-snapshot" | "last-prompt" | "system"
        ) {
            continue;
        }

        // Handle custom-title
        if msg_type == "custom-title" {
            if let Some(title) = parsed.custom_title {
                metadata.custom_title = Some(title);
            }
            continue;
        }

        // Handle agent-name
        if msg_type == "agent-name" {
            if metadata.custom_title.is_none() {
                if let Some(name) = parsed.agent_name {
                    metadata.custom_title = Some(name);
                }
            }
            continue;
        }

        let is_sidechain = parsed.is_sidechain.unwrap_or(false);

        // Handle user messages
        if msg_type == "user" {
            if parsed.is_meta.unwrap_or(false) {
                continue;
            }

            let message_val = parsed.message.as_ref();
            let content_val = message_val.and_then(|m| m.get("content"));

            let content_text = content_val
                .map(|c| extract_content_text_from_value(c))
                .unwrap_or_default();

            let is_tool_result = content_val
                .map(|c| c.is_array())
                .unwrap_or(false);

            // Track first prompt
            if metadata.first_prompt.is_none()
                && !is_sidechain
                && content_val.map(|c| c.is_string()).unwrap_or(false)
            {
                let prompt = content_val
                    .and_then(|c| c.as_str())
                    .unwrap_or("")
                    .chars()
                    .take(500)
                    .collect::<String>();
                metadata.first_prompt = Some(prompt);
            }

            // Track metadata from non-sidechain messages
            if !is_sidechain {
                if metadata.git_branch.is_none() {
                    if let Some(ref b) = parsed.git_branch {
                        if !b.is_empty() {
                            metadata.git_branch = Some(b.clone());
                        }
                    }
                }
                if metadata.cwd.is_none() {
                    metadata.cwd = parsed.cwd.clone();
                }
                if metadata.version.is_none() {
                    metadata.version = parsed.version.clone();
                }
                if metadata.permission_mode.is_none() {
                    metadata.permission_mode = parsed.permission_mode.clone();
                }
            }

            // Track timestamps
            if let Some(ref ts) = parsed.timestamp {
                if metadata
                    .created_at
                    .as_ref()
                    .map_or(true, |c| ts < c)
                {
                    metadata.created_at = Some(ts.clone());
                }
                if metadata
                    .modified_at
                    .as_ref()
                    .map_or(true, |m| ts > m)
                {
                    metadata.modified_at = Some(ts.clone());
                }
            }

            if !is_sidechain && !is_tool_result {
                metadata.user_message_count += 1;
            }

            let content_json = if is_tool_result {
                content_val.map(|c| serde_json::to_string(c).unwrap_or_default())
            } else {
                None
            };

            messages.push(ParsedMessage {
                uuid: parsed.uuid,
                parent_uuid: parsed.parent_uuid,
                msg_type: "user".to_string(),
                role: Some("user".to_string()),
                is_sidechain,
                agent_id: parsed.agent_id,
                model: None,
                content_text,
                content_json,
                has_tool_use: false,
                has_thinking: false,
                tool_names: Vec::new(),
                input_tokens: 0,
                output_tokens: 0,
                stop_reason: None,
                timestamp: parsed.timestamp,
                line_number,
            });
            continue;
        }

        // Handle assistant messages
        if msg_type == "assistant" {
            let inner: AssistantMessageInner = match parsed
                .message
                .as_ref()
                .and_then(|m| serde_json::from_value(m.clone()).ok())
            {
                Some(inner) => inner,
                None => continue,
            };

            let msg_id = inner.id.clone();

            // Skip streaming partials if we already have one
            if is_streaming_partial(&inner) {
                if let Some(ref id) = msg_id {
                    if assistant_message_ids.contains_key(id) {
                        continue;
                    }
                }
            }

            let content_blocks = inner.content.as_deref().unwrap_or(&[]);
            let (content_text, has_tool_use, has_thinking, tool_names) =
                extract_assistant_info(content_blocks);

            let usage = inner.usage.as_ref();
            let input_tokens = usage.and_then(|u| u.input_tokens).unwrap_or(0);
            let output_tokens = usage.and_then(|u| u.output_tokens).unwrap_or(0);
            let cache_creation = usage
                .and_then(|u| u.cache_creation_input_tokens)
                .unwrap_or(0);
            let cache_read = usage
                .and_then(|u| u.cache_read_input_tokens)
                .unwrap_or(0);

            let model = inner.model.clone();
            let stop_reason = inner.stop_reason.clone();

            let content_json = serde_json::to_string(content_blocks).ok();

            let parsed_msg = ParsedMessage {
                uuid: parsed.uuid,
                parent_uuid: parsed.parent_uuid,
                msg_type: "assistant".to_string(),
                role: Some("assistant".to_string()),
                is_sidechain,
                agent_id: parsed.agent_id,
                model: model.clone(),
                content_text,
                content_json,
                has_tool_use,
                has_thinking,
                tool_names: tool_names.clone(),
                input_tokens,
                output_tokens,
                stop_reason: stop_reason.clone(),
                timestamp: parsed.timestamp.clone(),
                line_number,
            };

            // Dedup: if we already have a message with same id, replace if this one is final
            if let Some(ref id) = msg_id {
                if let Some(&existing_idx) = assistant_message_ids.get(id) {
                    if stop_reason.is_some() {
                        messages[existing_idx] = parsed_msg;
                    }
                    continue;
                }
                assistant_message_ids.insert(id.clone(), messages.len());
            }

            // Track metadata
            if !is_sidechain {
                metadata.assistant_message_count += 1;
                if has_tool_use {
                    metadata.tool_use_count += tool_names.len() as i64;
                }
                metadata.total_input_tokens += input_tokens;
                metadata.total_output_tokens += output_tokens;
                metadata.total_cache_creation_tokens += cache_creation;
                metadata.total_cache_read_tokens += cache_read;

                if let Some(ref m) = model {
                    *metadata.models.entry(m.clone()).or_insert(0) += 1;
                }
            }

            if let Some(ref ts) = parsed.timestamp {
                if metadata
                    .modified_at
                    .as_ref()
                    .map_or(true, |m| ts > m)
                {
                    metadata.modified_at = Some(ts.clone());
                }
            }

            messages.push(parsed_msg);
        }
    }

    Ok(SessionParseResult { messages, metadata })
}

pub fn get_primary_model(models: &HashMap<String, i64>) -> Option<String> {
    models
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(model, _)| model.clone())
}

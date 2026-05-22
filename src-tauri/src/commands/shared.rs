use rusqlite::{params, Connection};

use crate::commands::replace::replace_in_string;

/// Replace the source project path with the target project path in a JSONL value.
/// Handles: `cwd` field, user message string content, assistant text blocks.
fn rewrite_project_path(val: &mut serde_json::Value, source_path: &str, target_path: &str) {
    if let Some(obj) = val.as_object_mut() {
        if let Some(cwd_val) = obj.get_mut("cwd") {
            if let Some(s) = cwd_val.as_str() {
                let (replaced, count) = replace_in_string(s, source_path, target_path, true);
                if count > 0 {
                    *cwd_val = serde_json::Value::String(replaced);
                }
            }
        }
    }

    let msg_type = match val.get("type").and_then(|t| t.as_str()) {
        Some(t) => t.to_string(),
        None => return,
    };

    match msg_type.as_str() {
        "user" => {
            if let Some(content) = val.get_mut("message").and_then(|m| m.get_mut("content")) {
                if let Some(s) = content.as_str() {
                    let (replaced, count) = replace_in_string(s, source_path, target_path, true);
                    if count > 0 {
                        *content = serde_json::Value::String(replaced);
                    }
                }
            }
        }
        "assistant" => {
            if let Some(content_arr) = val
                .get_mut("message")
                .and_then(|m| m.get_mut("content"))
                .and_then(|c| c.as_array_mut())
            {
                for block in content_arr.iter_mut() {
                    let block_type = block.get("type").and_then(|t| t.as_str()).unwrap_or("");
                    if block_type == "text" {
                        if let Some(text_val) = block.get_mut("text") {
                            if let Some(s) = text_val.as_str() {
                                let (replaced, count) =
                                    replace_in_string(s, source_path, target_path, true);
                                if count > 0 {
                                    *text_val = serde_json::Value::String(replaced);
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// Rewrite a single JSONL line: replace `sessionId` with `new_session_id` and
/// optionally rewrite source→target project paths in `cwd` and message text.
/// Empty lines and non-JSON lines are returned unchanged.
pub fn rewrite_jsonl_line(
    line: &str,
    new_session_id: &str,
    path_rewrite: Option<(&str, &str)>,
) -> Result<String, String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(String::new());
    }

    match serde_json::from_str::<serde_json::Value>(trimmed) {
        Ok(mut val) => {
            if let Some(obj) = val.as_object_mut() {
                if obj.contains_key("sessionId") {
                    obj.insert(
                        "sessionId".to_string(),
                        serde_json::Value::String(new_session_id.to_string()),
                    );
                }
            }

            if let Some((src, dst)) = path_rewrite {
                rewrite_project_path(&mut val, src, dst);
            }

            serde_json::to_string(&val).map_err(|e| format!("Failed to serialize: {}", e))
        }
        Err(_) => Ok(trimmed.to_string()),
    }
}

/// Duplicate session_metadata (is_favorite, notes) and session_tags from `src` to `dst`.
/// Failures are logged but not fatal — metadata is best-effort.
pub fn copy_session_metadata_and_tags(conn: &Connection, src: &str, dst: &str) {
    let _ = conn.execute(
        "INSERT INTO session_metadata (session_id, is_favorite, notes, updated_at)
         SELECT ?1, is_favorite, notes, datetime('now')
         FROM session_metadata WHERE session_id = ?2",
        params![dst, src],
    );

    let _ = conn.execute(
        "INSERT INTO session_tags (session_id, tag_id, created_at)
         SELECT ?1, tag_id, datetime('now')
         FROM session_tags WHERE session_id = ?2",
        params![dst, src],
    );
}

use std::fs;
use std::io::{BufRead, BufReader, Write};

use rusqlite::params;
use tauri::State;

use crate::commands::replace::replace_in_string;
use crate::db::DbPool;
use crate::sync::path_encoder::get_projects_dir;

/// Replace the source project path with the target project path in a JSONL value.
/// Handles: `cwd` field, user message string content, assistant text blocks.
fn rewrite_project_path(val: &mut serde_json::Value, source_path: &str, target_path: &str) {
    if let Some(obj) = val.as_object_mut() {
        // Replace in cwd field
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

#[tauri::command]
pub fn clone_session(
    session_id: String,
    target_project_id: i64,
    pool: State<'_, DbPool>,
) -> Result<String, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    // Look up the source session file path and project_id
    let (source_path, source_project_id): (String, i64) = conn
        .query_row(
            "SELECT file_path, project_id FROM sessions WHERE id = ?1",
            params![session_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("Session not found: {}", e))?;

    // Look up the target project encoded_name and path
    let (target_encoded, target_project_path): (String, String) = conn
        .query_row(
            "SELECT encoded_name, project_path FROM projects WHERE id = ?1",
            params![target_project_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("Target project not found: {}", e))?;

    // Look up the source project path for path rewriting
    let source_project_path: Option<String> = if source_project_id != target_project_id {
        conn.query_row(
            "SELECT project_path FROM projects WHERE id = ?1",
            params![source_project_id],
            |row| row.get(0),
        )
        .ok()
    } else {
        None
    };

    let source = std::path::Path::new(&source_path);
    if !source.exists() {
        return Err("Source session file not found on disk".to_string());
    }

    // Generate new session UUID
    let new_session_id = uuid::Uuid::new_v4().to_string();

    // Build target path
    let target_dir = get_projects_dir().join(&target_encoded);
    if !target_dir.exists() {
        return Err(format!(
            "Target project directory does not exist: {}",
            target_dir.display()
        ));
    }
    let target_path = target_dir.join(format!("{}.jsonl", new_session_id));

    // Read source, rewrite sessionId and project paths, write to temp file then rename
    let tmp_path = target_dir.join(format!(".{}.jsonl.tmp", new_session_id));
    {
        let src_file =
            fs::File::open(source).map_err(|e| format!("Failed to open source: {}", e))?;
        let reader = BufReader::new(src_file);
        let mut writer = fs::File::create(&tmp_path)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                writeln!(writer).map_err(|e| format!("Failed to write: {}", e))?;
                continue;
            }

            // Try to parse and rewrite sessionId + project paths
            match serde_json::from_str::<serde_json::Value>(trimmed) {
                Ok(mut val) => {
                    if let Some(obj) = val.as_object_mut() {
                        if obj.contains_key("sessionId") {
                            obj.insert(
                                "sessionId".to_string(),
                                serde_json::Value::String(new_session_id.clone()),
                            );
                        }
                    }

                    // Replace source project path with target in cwd and message content
                    if let Some(ref src_path) = source_project_path {
                        rewrite_project_path(&mut val, src_path, &target_project_path);
                    }

                    let rewritten = serde_json::to_string(&val)
                        .map_err(|e| format!("Failed to serialize: {}", e))?;
                    writeln!(writer, "{}", rewritten)
                        .map_err(|e| format!("Failed to write: {}", e))?;
                }
                Err(_) => {
                    // Not valid JSON, copy line as-is
                    writeln!(writer, "{}", trimmed)
                        .map_err(|e| format!("Failed to write: {}", e))?;
                }
            }
        }

        writer
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
    }

    // Atomic rename
    fs::rename(&tmp_path, &target_path)
        .map_err(|e| format!("Failed to rename temp file: {}", e))?;

    // Copy subagents if they exist
    let source_dir = source.parent().unwrap();
    let subagents_src = source_dir.join(&session_id).join("subagents");
    if subagents_src.exists() && subagents_src.is_dir() {
        let subagents_dst = target_dir.join(&new_session_id).join("subagents");
        fs::create_dir_all(&subagents_dst)
            .map_err(|e| format!("Failed to create subagents dir: {}", e))?;

        if let Ok(entries) = fs::read_dir(&subagents_src) {
            for entry in entries.filter_map(Result::ok) {
                let src_path = entry.path();
                let file_name = entry.file_name();
                let dst_path = subagents_dst.join(&file_name);
                fs::copy(&src_path, &dst_path).map_err(|e| {
                    format!(
                        "Failed to copy subagent file {}: {}",
                        file_name.to_string_lossy(),
                        e
                    )
                })?;
            }
        }
    }

    // Duplicate session_metadata if it exists
    let _ = conn.execute(
        "INSERT INTO session_metadata (session_id, is_favorite, notes, updated_at)
         SELECT ?1, is_favorite, notes, datetime('now')
         FROM session_metadata WHERE session_id = ?2",
        params![new_session_id, session_id],
    );

    // Duplicate session_tags
    let _ = conn.execute(
        "INSERT INTO session_tags (session_id, tag_id, created_at)
         SELECT ?1, tag_id, datetime('now')
         FROM session_tags WHERE session_id = ?2",
        params![new_session_id, session_id],
    );

    Ok(new_session_id)
}

use std::fs;
use std::io::{BufRead, BufReader, Write};

use rusqlite::params;
use tauri::State;

use crate::db::DbPool;
use crate::sync::path_encoder::get_projects_dir;

#[tauri::command]
pub fn clone_session(
    session_id: String,
    target_project_id: i64,
    pool: State<'_, DbPool>,
) -> Result<String, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    // Look up the source session file path
    let source_path: String = conn
        .query_row(
            "SELECT file_path FROM sessions WHERE id = ?1",
            params![session_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Session not found: {}", e))?;

    // Look up the target project encoded_name
    let target_encoded: String = conn
        .query_row(
            "SELECT encoded_name FROM projects WHERE id = ?1",
            params![target_project_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Target project not found: {}", e))?;

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

    // Read source, rewrite sessionId, write to temp file then rename
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

            // Try to parse and rewrite sessionId
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

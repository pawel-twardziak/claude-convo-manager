use std::fs;
use std::io::{BufRead, BufReader, Write};

use rusqlite::params;
use tauri::State;

use crate::commands::shared::{copy_session_metadata_and_tags, rewrite_jsonl_line};
use crate::db::DbPool;
use crate::sync::path_encoder::get_projects_dir;

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

        let path_rewrite = source_project_path
            .as_deref()
            .map(|src| (src, target_project_path.as_str()));

        for line in reader.lines() {
            let line = line.map_err(|e| format!("Failed to read line: {}", e))?;
            let rewritten = rewrite_jsonl_line(&line, &new_session_id, path_rewrite)?;
            writeln!(writer, "{}", rewritten).map_err(|e| format!("Failed to write: {}", e))?;
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

    copy_session_metadata_and_tags(&conn, &session_id, &new_session_id);

    Ok(new_session_id)
}

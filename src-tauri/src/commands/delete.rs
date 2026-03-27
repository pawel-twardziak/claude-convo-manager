use std::fs;

use rusqlite::params;
use tauri::State;

use crate::db::DbPool;

#[tauri::command]
pub fn delete_session(session_id: String, pool: State<'_, DbPool>) -> Result<(), String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    // Look up the file_path from sessions table
    let file_path: String = conn
        .query_row(
            "SELECT file_path FROM sessions WHERE id = ?1",
            params![session_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Session not found: {}", e))?;

    let path = std::path::Path::new(&file_path);

    // Delete the JSONL file
    if path.exists() {
        fs::remove_file(path).map_err(|e| format!("Failed to delete session file: {}", e))?;
    }

    // Delete subagents directory if it exists
    if let Some(parent) = path.parent() {
        let subagents_dir = parent.join(&session_id);
        if subagents_dir.exists() && subagents_dir.is_dir() {
            fs::remove_dir_all(&subagents_dir)
                .map_err(|e| format!("Failed to delete subagents directory: {}", e))?;
        }
    }

    // Delete from DB — messages, session_tags, session_metadata, subagents all cascade
    conn.execute("DELETE FROM sessions WHERE id = ?1", params![session_id])
        .map_err(|e| format!("Failed to delete session from database: {}", e))?;

    Ok(())
}

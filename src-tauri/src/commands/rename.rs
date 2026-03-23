use std::io::Write;
use tauri::State;

use crate::db::DbPool;

#[tauri::command]
pub fn rename_session(
    session_id: String,
    new_title: String,
    pool: State<'_, DbPool>,
) -> Result<(), String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    // Look up the file_path from sessions table
    let file_path: String = conn
        .query_row(
            "SELECT file_path FROM sessions WHERE id = ?1",
            rusqlite::params![session_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Session not found: {}", e))?;

    // Validate the file exists
    let path = std::path::Path::new(&file_path);
    if !path.exists() {
        return Err("Session file not found on disk".to_string());
    }

    // Build the custom-title JSONL line
    let title_line = serde_json::json!({
        "type": "custom-title",
        "customTitle": new_title,
        "sessionId": session_id,
    });
    let line = format!(
        "\n{}",
        serde_json::to_string(&title_line).map_err(|e| e.to_string())?
    );

    // Append to the JSONL file (file write first for consistency)
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .open(&file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    file.write_all(line.as_bytes())
        .map_err(|e| format!("Failed to write: {}", e))?;

    // Update the database
    conn.execute(
        "UPDATE sessions SET custom_title = ?1 WHERE id = ?2",
        rusqlite::params![new_title, session_id],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

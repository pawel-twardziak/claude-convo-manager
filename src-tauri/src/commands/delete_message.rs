use std::fs;
use std::io::{BufRead, BufReader, Write};

use rusqlite::params;
use tauri::State;

use crate::db::DbPool;
use crate::types::api::{DeleteMessagesFromLineParams, DeleteMessagesFromLineResult};

use super::replace::get_file_path;

#[tauri::command]
pub fn delete_messages_from_line(
    params: DeleteMessagesFromLineParams,
    pool: State<'_, DbPool>,
) -> Result<DeleteMessagesFromLineResult, String> {
    if params.from_line_number < 1 {
        return Err("from_line_number must be >= 1".to_string());
    }

    let conn = pool.get().map_err(|e| e.to_string())?;

    let file_path = get_file_path(&pool, &params.session_id)?;

    let src_file = fs::File::open(&file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(src_file);

    let path = std::path::Path::new(&file_path);
    let parent = path.parent().unwrap_or(path);
    let tmp_path = parent.join(format!(
        ".{}.delete.tmp",
        path.file_name().unwrap().to_string_lossy()
    ));

    {
        let mut writer = fs::File::create(&tmp_path)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        for (current_line, line_result) in (1_i64..).zip(reader.lines()) {
            let line = line_result.map_err(|e| format!("Failed to read line: {}", e))?;
            if current_line >= params.from_line_number {
                break;
            }
            writeln!(writer, "{}", line).map_err(|e| format!("Failed to write: {}", e))?;
        }

        writer
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
    }

    fs::rename(&tmp_path, &file_path).map_err(|e| format!("Failed to rename temp file: {}", e))?;

    let deleted_count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM messages WHERE session_id = ?1 AND line_number >= ?2",
            params![params.session_id, params.from_line_number],
            |row| row.get(0),
        )
        .map_err(|e| format!("Failed to count messages: {}", e))?;

    conn.execute(
        "DELETE FROM messages WHERE session_id = ?1 AND line_number >= ?2",
        params![params.session_id, params.from_line_number],
    )
    .map_err(|e| format!("Failed to delete messages: {}", e))?;

    conn.execute(
        "UPDATE sessions SET
            message_count = COALESCE(
                (SELECT COUNT(*) FROM messages
                 WHERE session_id = ?1 AND type IN ('user', 'assistant') AND is_sidechain = 0), 0),
            user_message_count = COALESCE(
                (SELECT COUNT(*) FROM messages
                 WHERE session_id = ?1 AND type = 'user' AND is_sidechain = 0), 0),
            assistant_message_count = COALESCE(
                (SELECT COUNT(*) FROM messages
                 WHERE session_id = ?1 AND type = 'assistant' AND is_sidechain = 0), 0),
            total_input_tokens = COALESCE(
                (SELECT SUM(input_tokens) FROM messages WHERE session_id = ?1), 0),
            total_output_tokens = COALESCE(
                (SELECT SUM(output_tokens) FROM messages WHERE session_id = ?1), 0),
            tool_use_count = COALESCE(
                (SELECT SUM(
                    CASE
                        WHEN tool_names IS NULL OR tool_names = '' THEN 0
                        ELSE LENGTH(tool_names) - LENGTH(REPLACE(tool_names, ',', '')) + 1
                    END
                ) FROM messages WHERE session_id = ?1), 0)
         WHERE id = ?1",
        params![params.session_id],
    )
    .map_err(|e| format!("Failed to update session counters: {}", e))?;

    log::info!(
        "delete_messages_from_line: session={}, from_line={}, deleted={}",
        params.session_id,
        params.from_line_number,
        deleted_count,
    );

    Ok(DeleteMessagesFromLineResult { deleted_count })
}

use std::fs;
use std::io::{BufRead, BufReader, Write};

use rusqlite::params;
use tauri::State;

use crate::db::DbPool;
use crate::types::api::{DeleteLastMessageParams, DeleteLastMessageResult};

use super::replace::get_file_path;

#[tauri::command]
pub fn delete_last_message(
    params: DeleteLastMessageParams,
    pool: State<'_, DbPool>,
) -> Result<DeleteLastMessageResult, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    // 1. Find the last visible message
    let row = conn.query_row(
        "SELECT id, line_number, type, input_tokens, output_tokens, tool_names
         FROM messages
         WHERE session_id = ?1 AND type IN ('user', 'assistant') AND is_sidechain = 0
         ORDER BY timestamp DESC, line_number DESC
         LIMIT 1",
        params![params.session_id],
        |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, Option<i64>>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, Option<String>>(5)?,
            ))
        },
    );

    let (msg_id, line_number_opt, msg_type, input_tokens, output_tokens, tool_names) = match row {
        Ok(r) => r,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Ok(DeleteLastMessageResult {
                deleted: false,
                deleted_message_id: None,
            });
        }
        Err(e) => return Err(format!("Failed to query last message: {}", e)),
    };

    let line_number = line_number_opt
        .ok_or("Cannot delete message: missing line number reference. Please re-sync first.")?;

    // 2. Get file path
    let file_path = get_file_path(&pool, &params.session_id)?;

    // 3. Atomic JSONL rewrite — skip the target line
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
            if current_line == line_number {
                continue;
            }
            writeln!(writer, "{}", line).map_err(|e| format!("Failed to write: {}", e))?;
        }

        writer
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
    }

    fs::rename(&tmp_path, &file_path).map_err(|e| format!("Failed to rename temp file: {}", e))?;

    // 4. Update DB after successful file rewrite

    // 4a. Delete the message row (FTS trigger cleans up automatically)
    conn.execute("DELETE FROM messages WHERE id = ?1", params![msg_id])
        .map_err(|e| format!("Failed to delete message: {}", e))?;

    // 4b. Shift line_numbers for messages after the deleted one
    conn.execute(
        "UPDATE messages SET line_number = line_number - 1
         WHERE session_id = ?1 AND line_number > ?2",
        params![params.session_id, line_number],
    )
    .map_err(|e| format!("Failed to update line numbers: {}", e))?;

    // 4c. Update session counters
    let tool_count = tool_names
        .as_deref()
        .filter(|s| !s.is_empty())
        .map(|s| s.split(',').count() as i64)
        .unwrap_or(0);

    conn.execute(
        "UPDATE sessions SET
            message_count = MAX(0, message_count - 1),
            user_message_count = CASE WHEN ?1 = 'user' THEN MAX(0, user_message_count - 1) ELSE user_message_count END,
            assistant_message_count = CASE WHEN ?1 = 'assistant' THEN MAX(0, assistant_message_count - 1) ELSE assistant_message_count END,
            total_input_tokens = MAX(0, total_input_tokens - ?2),
            total_output_tokens = MAX(0, total_output_tokens - ?3),
            tool_use_count = MAX(0, tool_use_count - ?4)
         WHERE id = ?5",
        params![
            msg_type,
            input_tokens,
            output_tokens,
            tool_count,
            params.session_id,
        ],
    )
    .map_err(|e| format!("Failed to update session counters: {}", e))?;

    log::info!(
        "delete_last_message: session={}, msg_id={}, line={}, type={}",
        params.session_id,
        msg_id,
        line_number,
        msg_type,
    );

    Ok(DeleteLastMessageResult {
        deleted: true,
        deleted_message_id: Some(msg_id),
    })
}

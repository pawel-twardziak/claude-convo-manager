use rusqlite::params_from_iter;
use tauri::State;

use crate::db::DbPool;
use crate::types::api::{GetMessagesParams, GetMessagesResponse};
use crate::types::db::MessageRow;

#[tauri::command]
pub fn get_session_messages(
    params: GetMessagesParams,
    pool: State<'_, DbPool>,
) -> Result<GetMessagesResponse, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(200);
    let exclude_sidechain = params.exclude_sidechain.unwrap_or(true);

    let mut conditions: Vec<String> = vec!["session_id = ?".to_string()];
    let mut bind_values: Vec<Box<dyn rusqlite::types::ToSql>> = vec![Box::new(params.session_id)];

    if exclude_sidechain {
        conditions.push("is_sidechain = 0".to_string());
    }
    conditions.push("type IN ('user', 'assistant')".to_string());

    let where_clause = format!("WHERE {}", conditions.join(" AND "));

    let count_sql = format!("SELECT COUNT(*) FROM messages {}", where_clause);
    let total: i64 = conn
        .query_row(&count_sql, params_from_iter(bind_values.iter()), |row| {
            row.get(0)
        })
        .map_err(|e| e.to_string())?;

    let data_sql = format!(
        "SELECT id, uuid, session_id, parent_uuid, type, role,
                is_sidechain, agent_id, model, content_text, content_json,
                has_tool_use, has_thinking, tool_names,
                input_tokens, output_tokens, stop_reason, timestamp, line_number
         FROM messages {}
         ORDER BY timestamp ASC, line_number ASC
         LIMIT ? OFFSET ?",
        where_clause
    );

    bind_values.push(Box::new(limit));
    bind_values.push(Box::new(offset));

    let mut stmt = conn.prepare(&data_sql).map_err(|e| e.to_string())?;
    let messages = stmt
        .query_map(params_from_iter(bind_values.iter()), |row| {
            Ok(MessageRow {
                id: row.get(0)?,
                uuid: row.get(1)?,
                session_id: row.get(2)?,
                parent_uuid: row.get(3)?,
                msg_type: row.get(4)?,
                role: row.get(5)?,
                is_sidechain: row.get(6)?,
                agent_id: row.get(7)?,
                model: row.get(8)?,
                content_text: row.get(9)?,
                content_json: row.get(10)?,
                has_tool_use: row.get(11)?,
                has_thinking: row.get(12)?,
                tool_names: row.get(13)?,
                input_tokens: row.get(14)?,
                output_tokens: row.get(15)?,
                stop_reason: row.get(16)?,
                timestamp: row.get(17)?,
                line_number: row.get(18)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    Ok(GetMessagesResponse {
        messages,
        total,
        has_more: offset + limit < total,
    })
}

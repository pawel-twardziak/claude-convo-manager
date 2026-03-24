use rusqlite::params_from_iter;
use tauri::State;

use crate::db::DbPool;
use crate::types::api::{SearchMessagesParams, SearchMessagesResponse};
use crate::types::db::SearchResult;

fn sanitize_fts_query(query: &str) -> String {
    query
        .replace(['\'', '"'], "")
        .split_whitespace()
        .filter(|w| !w.is_empty())
        .map(|word| format!("\"{}\"", word))
        .collect::<Vec<_>>()
        .join(" AND ")
}

#[tauri::command]
pub fn search_messages(
    params: SearchMessagesParams,
    pool: State<'_, DbPool>,
) -> Result<SearchMessagesResponse, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    let query = params.query.trim().to_string();
    if query.is_empty() {
        return Ok(SearchMessagesResponse {
            results: vec![],
            total: 0,
        });
    }

    let fts_query = sanitize_fts_query(&query);
    if fts_query.is_empty() {
        return Ok(SearchMessagesResponse {
            results: vec![],
            total: 0,
        });
    }

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    let project_filter = if params.project_id.is_some() {
        "AND s.project_id = ?"
    } else {
        ""
    };

    // Count
    let count_sql = format!(
        "SELECT COUNT(*) FROM messages_fts
         JOIN messages m ON m.id = messages_fts.rowid
         JOIN sessions s ON s.id = m.session_id
         WHERE messages_fts MATCH ? {}",
        project_filter
    );

    let mut count_params: Vec<Box<dyn rusqlite::types::ToSql>> = vec![Box::new(fts_query.clone())];
    if let Some(pid) = params.project_id {
        count_params.push(Box::new(pid));
    }

    let total: i64 = conn
        .query_row(&count_sql, params_from_iter(count_params.iter()), |row| {
            row.get(0)
        })
        .unwrap_or(0);

    // Data
    let data_sql = format!(
        "SELECT m.id as message_id, m.session_id,
                p.display_name as project_display_name,
                COALESCE(s.custom_title, s.first_prompt) as session_title,
                snippet(messages_fts, 0, '<mark>', '</mark>', '...', 40) as snippet,
                m.timestamp, rank
         FROM messages_fts
         JOIN messages m ON m.id = messages_fts.rowid
         JOIN sessions s ON s.id = m.session_id
         JOIN projects p ON p.id = s.project_id
         WHERE messages_fts MATCH ? {}
         ORDER BY rank
         LIMIT ? OFFSET ?",
        project_filter
    );

    let mut data_params: Vec<Box<dyn rusqlite::types::ToSql>> = vec![Box::new(fts_query)];
    if let Some(pid) = params.project_id {
        data_params.push(Box::new(pid));
    }
    data_params.push(Box::new(limit));
    data_params.push(Box::new(offset));

    let mut stmt = conn.prepare(&data_sql).map_err(|e| e.to_string())?;
    let results = stmt
        .query_map(params_from_iter(data_params.iter()), |row| {
            Ok(SearchResult {
                message_id: row.get(0)?,
                session_id: row.get(1)?,
                project_display_name: row.get(2)?,
                session_title: row.get(3)?,
                snippet: row.get(4)?,
                timestamp: row.get(5)?,
                rank: row.get(6)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    Ok(SearchMessagesResponse { results, total })
}

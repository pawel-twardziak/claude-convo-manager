use rusqlite::params_from_iter;
use tauri::State;

use crate::db::DbPool;
use crate::types::api::{ActivityEntry, GetTokenUsageParams, ProjectBreakdownEntry, TokenUsageEntry};
use crate::types::db::DashboardStats;

#[tauri::command]
pub fn get_dashboard_stats(pool: State<'_, DbPool>) -> Result<DashboardStats, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    conn.query_row(
        "SELECT
            COUNT(*) as total_sessions,
            (SELECT COUNT(*) FROM projects) as total_projects,
            (SELECT COUNT(*) FROM messages) as total_messages,
            COALESCE(SUM(total_input_tokens), 0) as total_input_tokens,
            COALESCE(SUM(total_output_tokens), 0) as total_output_tokens,
            COALESCE(SUM(total_cache_creation_tokens), 0) as total_cache_creation_tokens,
            COALESCE(SUM(total_cache_read_tokens), 0) as total_cache_read_tokens,
            COALESCE(SUM(estimated_cost_usd), 0) as estimated_total_cost,
            COALESCE(SUM(CASE WHEN is_active = 1 THEN 1 ELSE 0 END), 0) as active_sessions,
            COALESCE(SUM(CASE WHEN date(created_at) = date('now') THEN 1 ELSE 0 END), 0) as today_sessions,
            COALESCE(AVG(message_count), 0) as avg_message_count
         FROM sessions",
        [],
        |row| {
            Ok(DashboardStats {
                total_sessions: row.get(0)?,
                total_projects: row.get(1)?,
                total_messages: row.get(2)?,
                total_input_tokens: row.get(3)?,
                total_output_tokens: row.get(4)?,
                total_cache_creation_tokens: row.get(5)?,
                total_cache_read_tokens: row.get(6)?,
                estimated_total_cost: row.get(7)?,
                active_sessions: row.get(8)?,
                today_sessions: row.get(9)?,
                avg_message_count: row.get(10)?,
            })
        },
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_token_usage_over_time(
    params: GetTokenUsageParams,
    pool: State<'_, DbPool>,
) -> Result<Vec<TokenUsageEntry>, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    let group_by = params.group_by.as_deref().unwrap_or("day");
    let date_format = match group_by {
        "month" => "%Y-%m",
        "week" => "%Y-W%W",
        _ => "%Y-%m-%d",
    };

    let mut conditions: Vec<String> = vec!["created_at IS NOT NULL".to_string()];
    let mut bind_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref date_from) = params.date_from {
        conditions.push("created_at >= ?".to_string());
        bind_values.push(Box::new(date_from.clone()));
    }
    if let Some(ref date_to) = params.date_to {
        conditions.push("created_at <= ?".to_string());
        bind_values.push(Box::new(date_to.clone()));
    }
    if let Some(pid) = params.project_id {
        conditions.push("project_id = ?".to_string());
        bind_values.push(Box::new(pid));
    }

    let sql = format!(
        "SELECT strftime('{}', created_at) as date,
                SUM(total_input_tokens) as input_tokens,
                SUM(total_output_tokens) as output_tokens,
                SUM(estimated_cost_usd) as cost,
                COUNT(*) as session_count
         FROM sessions
         WHERE {}
         GROUP BY strftime('{}', created_at)
         ORDER BY date ASC",
        date_format,
        conditions.join(" AND "),
        date_format
    );

    let mut stmt = conn.prepare(&sql).map_err(|e| e.to_string())?;
    let entries = stmt
        .query_map(params_from_iter(bind_values.iter()), |row| {
            Ok(TokenUsageEntry {
                date: row.get(0)?,
                input_tokens: row.get(1)?,
                output_tokens: row.get(2)?,
                cost: row.get(3)?,
                session_count: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    Ok(entries)
}

#[tauri::command]
pub fn get_project_breakdown(pool: State<'_, DbPool>) -> Result<Vec<ProjectBreakdownEntry>, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT p.display_name as name, p.session_count as sessions, p.total_tokens as tokens
             FROM projects p
             WHERE p.session_count > 0
             ORDER BY p.session_count DESC
             LIMIT 15",
        )
        .map_err(|e| e.to_string())?;

    let entries = stmt
        .query_map([], |row| {
            Ok(ProjectBreakdownEntry {
                name: row.get(0)?,
                sessions: row.get(1)?,
                tokens: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    Ok(entries)
}

#[tauri::command]
pub fn get_activity_data(pool: State<'_, DbPool>) -> Result<Vec<ActivityEntry>, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare(
            "SELECT date(created_at) as date, COUNT(*) as count
             FROM sessions
             WHERE created_at >= date('now', '-90 days')
             GROUP BY date(created_at)
             ORDER BY date ASC",
        )
        .map_err(|e| e.to_string())?;

    let entries = stmt
        .query_map([], |row| {
            Ok(ActivityEntry {
                date: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    Ok(entries)
}

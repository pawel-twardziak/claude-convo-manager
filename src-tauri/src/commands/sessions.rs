use rusqlite::params_from_iter;
use tauri::State;

use crate::db::DbPool;
use crate::types::api::{GetSessionsParams, GetSessionsResponse};
use crate::types::db::{DateRange, FilterOptionProject, FilterOptions, SessionWithProject};

#[tauri::command]
pub fn get_sessions(
    params: GetSessionsParams,
    pool: State<'_, DbPool>,
) -> Result<GetSessionsResponse, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    let sort_by = params.sort_by.as_deref().unwrap_or("modified_at");
    let sort_dir = params.sort_dir.as_deref().unwrap_or("desc");
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(30);

    let allowed_sorts = [
        "created_at",
        "modified_at",
        "message_count",
        "file_size",
        "total_input_tokens",
        "estimated_cost_usd",
    ];
    let safe_sort = if allowed_sorts.contains(&sort_by) {
        sort_by
    } else {
        "modified_at"
    };
    let safe_dir = if sort_dir == "asc" { "ASC" } else { "DESC" };
    let offset = (page - 1) * page_size;

    let mut conditions: Vec<String> = Vec::new();
    let mut bind_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(pid) = params.project_id {
        conditions.push("s.project_id = ?".to_string());
        bind_values.push(Box::new(pid));
    }
    if let Some(ref branch) = params.git_branch {
        conditions.push("s.git_branch = ?".to_string());
        bind_values.push(Box::new(branch.clone()));
    }
    if let Some(ref model) = params.model {
        conditions.push("s.model = ?".to_string());
        bind_values.push(Box::new(model.clone()));
    }
    if let Some(ref date_from) = params.date_from {
        conditions.push("s.created_at >= ?".to_string());
        bind_values.push(Box::new(date_from.clone()));
    }
    if let Some(ref date_to) = params.date_to {
        conditions.push("s.created_at <= ?".to_string());
        bind_values.push(Box::new(date_to.clone()));
    }
    if let Some(ref search) = params.search {
        conditions.push("(s.first_prompt LIKE ? OR s.custom_title LIKE ?)".to_string());
        let like = format!("%{}%", search);
        bind_values.push(Box::new(like.clone()));
        bind_values.push(Box::new(like));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count
    let count_sql = format!("SELECT COUNT(*) as total FROM sessions s {}", where_clause);
    let total: i64 = conn
        .query_row(&count_sql, params_from_iter(bind_values.iter()), |row| {
            row.get(0)
        })
        .map_err(|e| e.to_string())?;

    // Data
    let data_sql = format!(
        "SELECT s.id, s.project_id, s.file_path, s.file_mtime, s.file_size,
                s.first_prompt, s.custom_title, s.message_count,
                s.user_message_count, s.assistant_message_count, s.tool_use_count,
                s.git_branch, s.cwd, s.model, s.version, s.permission_mode,
                s.is_sidechain, s.is_active,
                s.total_input_tokens, s.total_output_tokens,
                s.total_cache_creation_tokens, s.total_cache_read_tokens,
                s.estimated_cost_usd, s.created_at, s.modified_at, s.synced_at,
                p.project_path, p.display_name as project_display_name
         FROM sessions s
         JOIN projects p ON p.id = s.project_id
         {} ORDER BY s.{} {} LIMIT ? OFFSET ?",
        where_clause, safe_sort, safe_dir
    );

    bind_values.push(Box::new(page_size));
    bind_values.push(Box::new(offset));

    let mut stmt = conn.prepare(&data_sql).map_err(|e| e.to_string())?;
    let sessions = stmt
        .query_map(params_from_iter(bind_values.iter()), |row| {
            Ok(SessionWithProject {
                id: row.get(0)?,
                project_id: row.get(1)?,
                file_path: row.get(2)?,
                file_mtime: row.get(3)?,
                file_size: row.get(4)?,
                first_prompt: row.get(5)?,
                custom_title: row.get(6)?,
                message_count: row.get(7)?,
                user_message_count: row.get(8)?,
                assistant_message_count: row.get(9)?,
                tool_use_count: row.get(10)?,
                git_branch: row.get(11)?,
                cwd: row.get(12)?,
                model: row.get(13)?,
                version: row.get(14)?,
                permission_mode: row.get(15)?,
                is_sidechain: row.get(16)?,
                is_active: row.get(17)?,
                total_input_tokens: row.get(18)?,
                total_output_tokens: row.get(19)?,
                total_cache_creation_tokens: row.get(20)?,
                total_cache_read_tokens: row.get(21)?,
                estimated_cost_usd: row.get(22)?,
                created_at: row.get(23)?,
                modified_at: row.get(24)?,
                synced_at: row.get(25)?,
                project_path: row.get(26)?,
                project_display_name: row.get(27)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    Ok(GetSessionsResponse {
        sessions,
        total,
        page,
        page_size,
    })
}

#[tauri::command]
pub fn get_session(
    session_id: String,
    pool: State<'_, DbPool>,
) -> Result<Option<SessionWithProject>, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    let result = conn
        .query_row(
            "SELECT s.id, s.project_id, s.file_path, s.file_mtime, s.file_size,
                    s.first_prompt, s.custom_title, s.message_count,
                    s.user_message_count, s.assistant_message_count, s.tool_use_count,
                    s.git_branch, s.cwd, s.model, s.version, s.permission_mode,
                    s.is_sidechain, s.is_active,
                    s.total_input_tokens, s.total_output_tokens,
                    s.total_cache_creation_tokens, s.total_cache_read_tokens,
                    s.estimated_cost_usd, s.created_at, s.modified_at, s.synced_at,
                    p.project_path, p.display_name as project_display_name
             FROM sessions s
             JOIN projects p ON p.id = s.project_id
             WHERE s.id = ?1",
            rusqlite::params![session_id],
            |row| {
                Ok(SessionWithProject {
                    id: row.get(0)?,
                    project_id: row.get(1)?,
                    file_path: row.get(2)?,
                    file_mtime: row.get(3)?,
                    file_size: row.get(4)?,
                    first_prompt: row.get(5)?,
                    custom_title: row.get(6)?,
                    message_count: row.get(7)?,
                    user_message_count: row.get(8)?,
                    assistant_message_count: row.get(9)?,
                    tool_use_count: row.get(10)?,
                    git_branch: row.get(11)?,
                    cwd: row.get(12)?,
                    model: row.get(13)?,
                    version: row.get(14)?,
                    permission_mode: row.get(15)?,
                    is_sidechain: row.get(16)?,
                    is_active: row.get(17)?,
                    total_input_tokens: row.get(18)?,
                    total_output_tokens: row.get(19)?,
                    total_cache_creation_tokens: row.get(20)?,
                    total_cache_read_tokens: row.get(21)?,
                    estimated_cost_usd: row.get(22)?,
                    created_at: row.get(23)?,
                    modified_at: row.get(24)?,
                    synced_at: row.get(25)?,
                    project_path: row.get(26)?,
                    project_display_name: row.get(27)?,
                })
            },
        )
        .ok();

    Ok(result)
}

#[tauri::command]
pub fn get_filter_options(pool: State<'_, DbPool>) -> Result<FilterOptions, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    let mut stmt = conn
        .prepare("SELECT id, display_name, session_count FROM projects ORDER BY session_count DESC")
        .map_err(|e| e.to_string())?;
    let projects: Vec<FilterOptionProject> = stmt
        .query_map([], |row| {
            Ok(FilterOptionProject {
                id: row.get(0)?,
                display_name: row.get(1)?,
                session_count: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    let mut stmt = conn
        .prepare(
            "SELECT DISTINCT git_branch FROM sessions WHERE git_branch IS NOT NULL AND git_branch != '' ORDER BY git_branch",
        )
        .map_err(|e| e.to_string())?;
    let branches: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    let mut stmt = conn
        .prepare("SELECT DISTINCT model FROM sessions WHERE model IS NOT NULL ORDER BY model")
        .map_err(|e| e.to_string())?;
    let models: Vec<String> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    let date_range = conn
        .query_row(
            "SELECT MIN(created_at), MAX(created_at) FROM sessions",
            [],
            |row| {
                Ok(DateRange {
                    min: row.get(0)?,
                    max: row.get(1)?,
                })
            },
        )
        .unwrap_or(DateRange {
            min: None,
            max: None,
        });

    Ok(FilterOptions {
        projects,
        branches,
        models,
        date_range,
    })
}

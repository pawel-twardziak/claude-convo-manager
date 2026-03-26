use rusqlite::params_from_iter;
use tauri::State;

use crate::db::DbPool;
use crate::types::api::{GetProjectsParams, GetProjectsResponse};
use crate::types::db::ProjectWithStats;

#[tauri::command]
pub fn get_projects(
    params: GetProjectsParams,
    pool: State<'_, DbPool>,
) -> Result<GetProjectsResponse, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;

    let sort_by = params.sort_by.as_deref().unwrap_or("last_activity_at");
    let sort_dir = params.sort_dir.as_deref().unwrap_or("desc");
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(30);

    let allowed_sorts = [
        "display_name",
        "session_count",
        "total_tokens",
        "estimated_cost_usd",
        "last_activity_at",
        "created_at",
    ];
    let safe_sort = if allowed_sorts.contains(&sort_by) {
        sort_by
    } else {
        "last_activity_at"
    };
    let safe_dir = if sort_dir == "asc" { "ASC" } else { "DESC" };
    let offset = (page - 1) * page_size;

    let mut conditions: Vec<String> = Vec::new();
    let mut bind_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(ref search) = params.search {
        conditions.push("(p.display_name LIKE ? OR p.project_path LIKE ?)".to_string());
        let like = format!("%{}%", search);
        bind_values.push(Box::new(like.clone()));
        bind_values.push(Box::new(like));
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // For sorting by computed columns, we need to map to the appropriate expression
    let order_expr = match safe_sort {
        "estimated_cost_usd" => "COALESCE(SUM(s.estimated_cost_usd), 0.0)".to_string(),
        "display_name" => "p.display_name".to_string(),
        _ => format!("p.{}", safe_sort),
    };

    // Count
    let count_sql = format!("SELECT COUNT(*) FROM projects p {}", where_clause);
    let total: i64 = conn
        .query_row(&count_sql, params_from_iter(bind_values.iter()), |row| {
            row.get(0)
        })
        .map_err(|e| e.to_string())?;

    // Data — duplicate bind values since they're consumed by count query
    let mut data_bind: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    if let Some(ref search) = params.search {
        let like = format!("%{}%", search);
        data_bind.push(Box::new(like.clone()));
        data_bind.push(Box::new(like));
    }

    let data_sql = format!(
        "SELECT p.id, p.encoded_name, p.project_path, p.display_name,
                p.session_count, p.total_tokens, p.last_activity_at, p.created_at,
                COALESCE(SUM(s.total_input_tokens), 0),
                COALESCE(SUM(s.total_output_tokens), 0),
                COALESCE(SUM(s.total_cache_creation_tokens), 0),
                COALESCE(SUM(s.total_cache_read_tokens), 0),
                COALESCE(SUM(s.estimated_cost_usd), 0.0),
                GROUP_CONCAT(DISTINCT s.model),
                GROUP_CONCAT(DISTINCT s.git_branch)
         FROM projects p
         LEFT JOIN sessions s ON s.project_id = p.id
         {} GROUP BY p.id ORDER BY {} {} LIMIT ? OFFSET ?",
        where_clause, order_expr, safe_dir
    );

    data_bind.push(Box::new(page_size));
    data_bind.push(Box::new(offset));

    let mut stmt = conn.prepare(&data_sql).map_err(|e| e.to_string())?;
    let projects = stmt
        .query_map(params_from_iter(data_bind.iter()), |row| {
            Ok(ProjectWithStats {
                id: row.get(0)?,
                encoded_name: row.get(1)?,
                project_path: row.get(2)?,
                display_name: row.get(3)?,
                session_count: row.get(4)?,
                total_tokens: row.get(5)?,
                last_activity_at: row.get(6)?,
                created_at: row.get(7)?,
                total_input_tokens: row.get(8)?,
                total_output_tokens: row.get(9)?,
                total_cache_creation_tokens: row.get(10)?,
                total_cache_read_tokens: row.get(11)?,
                estimated_cost_usd: row.get(12)?,
                distinct_models: row.get(13)?,
                distinct_branches: row.get(14)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .collect();

    Ok(GetProjectsResponse {
        projects,
        total,
        page,
        page_size,
    })
}

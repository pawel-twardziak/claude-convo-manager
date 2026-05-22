use std::fs;
use std::io::{BufRead, BufReader, Write};

use rusqlite::params;
use tauri::State;

use crate::commands::shared::{copy_session_metadata_and_tags, rewrite_jsonl_line};
use crate::db::DbPool;
use crate::sync::path_encoder::get_projects_dir;
use crate::types::api::{ForkSessionParams, ForkSessionResult};

/// Detect whether the last copied line is an assistant message containing a
/// tool_use block. Such a fork ends mid-tool-sequence: the matching tool_result
/// (which would be on a later line) is missing, so the resulting session may
/// not be resumable cleanly by Claude Code.
fn ends_with_unresolved_tool_use(line: &str) -> bool {
    let Ok(val) = serde_json::from_str::<serde_json::Value>(line) else {
        return false;
    };
    let msg_type = val.get("type").and_then(|t| t.as_str()).unwrap_or("");
    if msg_type != "assistant" {
        return false;
    }
    let Some(content) = val
        .get("message")
        .and_then(|m| m.get("content"))
        .and_then(|c| c.as_array())
    else {
        return false;
    };
    content
        .iter()
        .any(|b| b.get("type").and_then(|t| t.as_str()) == Some("tool_use"))
}

#[tauri::command]
pub fn fork_session_from_line(
    params: ForkSessionParams,
    pool: State<'_, DbPool>,
) -> Result<ForkSessionResult, String> {
    if params.up_to_line_number < 1 {
        return Err("up_to_line_number must be >= 1".to_string());
    }

    let conn = pool.get().map_err(|e| e.to_string())?;

    // Look up source session: file path + parent project
    let (source_path, source_project_id): (String, i64) = conn
        .query_row(
            "SELECT file_path, project_id FROM sessions WHERE id = ?1",
            params![params.session_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("Session not found: {}", e))?;

    let target_project_id = params.target_project_id.unwrap_or(source_project_id);

    // Look up target project encoded_name + path
    let (target_encoded, target_project_path): (String, String) = conn
        .query_row(
            "SELECT encoded_name, project_path FROM projects WHERE id = ?1",
            params![target_project_id],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .map_err(|e| format!("Target project not found: {}", e))?;

    // Only rewrite paths if forking across projects
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

    let new_session_id = uuid::Uuid::new_v4().to_string();

    let target_dir = get_projects_dir().join(&target_encoded);
    if !target_dir.exists() {
        return Err(format!(
            "Target project directory does not exist: {}",
            target_dir.display()
        ));
    }
    let target_path = target_dir.join(format!("{}.jsonl", new_session_id));
    let tmp_path = target_dir.join(format!(".{}.jsonl.tmp", new_session_id));

    let path_rewrite = source_project_path
        .as_deref()
        .map(|src| (src, target_project_path.as_str()));

    let mut written_lines: i64 = 0;
    let mut last_line: Option<String> = None;

    {
        let src_file =
            fs::File::open(source).map_err(|e| format!("Failed to open source: {}", e))?;
        let reader = BufReader::new(src_file);
        let mut writer = fs::File::create(&tmp_path)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        for (current_line, line_result) in (1_i64..).zip(reader.lines()) {
            if current_line > params.up_to_line_number {
                break;
            }
            let line = line_result.map_err(|e| format!("Failed to read line: {}", e))?;
            let rewritten = rewrite_jsonl_line(&line, &new_session_id, path_rewrite)?;
            writeln!(writer, "{}", rewritten).map_err(|e| format!("Failed to write: {}", e))?;
            written_lines = current_line;
            last_line = Some(line);
        }

        writer
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
    }

    if written_lines == 0 {
        let _ = fs::remove_file(&tmp_path);
        return Err("Source file is empty — nothing to fork".to_string());
    }

    fs::rename(&tmp_path, &target_path)
        .map_err(|e| format!("Failed to rename temp file: {}", e))?;

    // Stamp lineage on the sessions row. INSERT may conflict if a concurrent
    // sync already discovered the new JSONL — in that case just patch the
    // lineage columns; sync will keep counters/title accurate on the next pass.
    conn.execute(
        "INSERT INTO sessions (id, project_id, file_path, forked_from_session_id, forked_at_line_number, synced_at)
         VALUES (?1, ?2, ?3, ?4, ?5, datetime('now'))
         ON CONFLICT(id) DO UPDATE SET
             forked_from_session_id = excluded.forked_from_session_id,
             forked_at_line_number = excluded.forked_at_line_number",
        params![
            new_session_id,
            target_project_id,
            target_path.to_string_lossy().to_string(),
            params.session_id,
            params.up_to_line_number,
        ],
    )
    .map_err(|e| format!("Failed to record fork lineage: {}", e))?;

    copy_session_metadata_and_tags(&conn, &params.session_id, &new_session_id);

    let warning = last_line
        .as_deref()
        .filter(|l| ends_with_unresolved_tool_use(l))
        .map(|_| "fork_point_mid_tool_use".to_string());

    log::info!(
        "fork_session_from_line: source={}, new={}, lines={}, warning={:?}",
        params.session_id,
        new_session_id,
        written_lines,
        warning,
    );

    Ok(ForkSessionResult {
        new_session_id,
        new_line_count: written_lines,
        warning,
    })
}

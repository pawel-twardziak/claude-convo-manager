use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

use rusqlite::params;
use tauri::{AppHandle, Emitter};

use crate::db::DbPool;
use crate::types::api::SyncProgress;
use crate::types::claude::{ActiveSessionFile, ClaudeHistoryEntry, SessionsIndexFile, SubagentMeta};

use super::parsers::{get_primary_model, parse_session_file};
use super::path_encoder::{extract_display_name, get_claude_dir, get_projects_dir};
use super::token_calculator::estimate_cost;

fn emit_progress(app: &AppHandle, phase: &str, current: i64, total: i64) {
    let _ = app.emit(
        "sync-progress",
        SyncProgress {
            phase: phase.to_string(),
            current,
            total,
        },
    );
}

pub fn full_sync(pool: &DbPool, app: &AppHandle) -> Result<(i64, i64), String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    let claude_dir = get_claude_dir();
    let projects_dir = get_projects_dir();

    // Phase 1: Parse history.jsonl to build project path map
    emit_progress(app, "Reading history", 0, 1);
    let mut project_path_map = std::collections::HashMap::<String, String>::new();
    let history_path = claude_dir.join("history.jsonl");

    if history_path.exists() {
        if let Ok(file) = fs::File::open(&history_path) {
            let reader = BufReader::new(file);
            for line in reader.lines().map_while(Result::ok) {
                let trimmed = line.trim().to_string();
                if trimmed.is_empty() {
                    continue;
                }
                if let Ok(entry) = serde_json::from_str::<ClaudeHistoryEntry>(&trimmed) {
                    if let Some(project) = entry.project {
                        let encoded = project.replace(|c: char| c == '/' || c == '.', "-");
                        project_path_map.insert(encoded, project);
                    }
                }
            }
        }
    }

    // Phase 2: Discover project directories
    emit_progress(app, "Discovering projects", 0, 1);
    if !projects_dir.exists() {
        return Ok((0, 0));
    }

    let project_dirs: Vec<String> = fs::read_dir(&projects_dir)
        .map_err(|e| e.to_string())?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_dir() {
                Some(entry.file_name().to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    // Phase 3: Create/update project records
    {
        let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
        for encoded_name in &project_dirs {
            let project_path = project_path_map
                .get(encoded_name)
                .cloned()
                .unwrap_or_else(|| {
                    format!(
                        "/{}",
                        encoded_name
                            .strip_prefix('-')
                            .unwrap_or(encoded_name)
                            .replace('-', "/")
                    )
                });
            let display_name = extract_display_name(&project_path);
            tx.execute(
                "INSERT INTO projects (encoded_name, project_path, display_name)
                 VALUES (?1, ?2, ?3)
                 ON CONFLICT(encoded_name) DO UPDATE SET
                   project_path = excluded.project_path,
                   display_name = excluded.display_name",
                params![encoded_name, project_path, display_name],
            )
            .map_err(|e| e.to_string())?;
        }
        tx.commit().map_err(|e| e.to_string())?;
    }

    // Enrich from sessions-index.json
    for encoded_name in &project_dirs {
        let index_path = projects_dir.join(encoded_name).join("sessions-index.json");
        if index_path.exists() {
            if let Ok(content) = fs::read_to_string(&index_path) {
                if let Ok(index_data) = serde_json::from_str::<SessionsIndexFile>(&content) {
                    if let Some(entries) = &index_data.entries {
                        if let Some(first) = entries.first() {
                            if let Some(ref real_path) = first.project_path {
                                let display_name = extract_display_name(real_path);
                                let _ = conn.execute(
                                    "INSERT INTO projects (encoded_name, project_path, display_name)
                                     VALUES (?1, ?2, ?3)
                                     ON CONFLICT(encoded_name) DO UPDATE SET
                                       project_path = excluded.project_path,
                                       display_name = excluded.display_name",
                                    params![encoded_name, real_path, display_name],
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    // Phase 4: Discover and parse session files
    struct SessionFile {
        project_encoded: String,
        session_id: String,
        file_path: PathBuf,
    }

    let mut session_files: Vec<SessionFile> = Vec::new();
    for encoded_name in &project_dirs {
        let project_dir = projects_dir.join(encoded_name);
        if let Ok(entries) = fs::read_dir(&project_dir) {
            for entry in entries.filter_map(Result::ok) {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.ends_with(".jsonl") {
                    let session_id = name.trim_end_matches(".jsonl").to_string();
                    session_files.push(SessionFile {
                        project_encoded: encoded_name.clone(),
                        session_id,
                        file_path: entry.path(),
                    });
                }
            }
        }
    }

    let total_sessions = session_files.len() as i64;
    emit_progress(app, "Parsing sessions", 0, total_sessions);

    let mut total_messages: i64 = 0;

    for (i, sf) in session_files.iter().enumerate() {
        if i % 10 == 0 {
            emit_progress(app, "Parsing sessions", i as i64, total_sessions);
        }

        let project_id: Option<i64> = conn
            .query_row(
                "SELECT id FROM projects WHERE encoded_name = ?1",
                params![sf.project_encoded],
                |row| row.get(0),
            )
            .ok();

        let project_id = match project_id {
            Some(id) => id,
            None => continue,
        };

        let stat = match fs::metadata(&sf.file_path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        let mtime_ms = stat
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs_f64() * 1000.0)
            .unwrap_or(0.0);
        let file_size = stat.len() as i64;

        let result = match parse_session_file(&sf.file_path) {
            Ok(r) => r,
            Err(e) => {
                log::error!("Error parsing {}: {}", sf.file_path.display(), e);
                continue;
            }
        };

        let primary_model = get_primary_model(&result.metadata.models);
        let cost = primary_model
            .as_ref()
            .map(|m| {
                estimate_cost(
                    m,
                    result.metadata.total_input_tokens,
                    result.metadata.total_output_tokens,
                    result.metadata.total_cache_creation_tokens,
                    result.metadata.total_cache_read_tokens,
                )
            })
            .unwrap_or(0.0);

        let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
        let insert_result = (|| -> Result<(), rusqlite::Error> {
            tx.execute(
                "INSERT INTO sessions (
                    id, project_id, file_path, file_mtime, file_size,
                    first_prompt, custom_title, message_count,
                    user_message_count, assistant_message_count, tool_use_count,
                    git_branch, cwd, model, version, permission_mode,
                    is_sidechain, total_input_tokens, total_output_tokens,
                    total_cache_creation_tokens, total_cache_read_tokens,
                    estimated_cost_usd, created_at, modified_at
                ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18,?19,?20,?21,?22,?23,?24)
                ON CONFLICT(id) DO UPDATE SET
                    file_mtime=excluded.file_mtime, file_size=excluded.file_size,
                    first_prompt=excluded.first_prompt, custom_title=excluded.custom_title,
                    message_count=excluded.message_count,
                    user_message_count=excluded.user_message_count,
                    assistant_message_count=excluded.assistant_message_count,
                    tool_use_count=excluded.tool_use_count,
                    git_branch=excluded.git_branch, cwd=excluded.cwd,
                    model=excluded.model, version=excluded.version,
                    permission_mode=excluded.permission_mode,
                    total_input_tokens=excluded.total_input_tokens,
                    total_output_tokens=excluded.total_output_tokens,
                    total_cache_creation_tokens=excluded.total_cache_creation_tokens,
                    total_cache_read_tokens=excluded.total_cache_read_tokens,
                    estimated_cost_usd=excluded.estimated_cost_usd,
                    created_at=excluded.created_at, modified_at=excluded.modified_at,
                    synced_at=datetime('now')",
                params![
                    sf.session_id,
                    project_id,
                    sf.file_path.to_string_lossy().to_string(),
                    mtime_ms,
                    file_size,
                    result.metadata.first_prompt,
                    result.metadata.custom_title,
                    result.messages.len() as i64,
                    result.metadata.user_message_count,
                    result.metadata.assistant_message_count,
                    result.metadata.tool_use_count,
                    result.metadata.git_branch,
                    result.metadata.cwd,
                    primary_model,
                    result.metadata.version,
                    result.metadata.permission_mode,
                    0i64, // is_sidechain
                    result.metadata.total_input_tokens,
                    result.metadata.total_output_tokens,
                    result.metadata.total_cache_creation_tokens,
                    result.metadata.total_cache_read_tokens,
                    cost,
                    result.metadata.created_at,
                    result.metadata.modified_at,
                ],
            )?;

            // Delete old messages for re-sync
            tx.execute(
                "DELETE FROM messages WHERE session_id = ?1",
                params![sf.session_id],
            )?;

            // Insert messages
            let mut stmt = tx.prepare(
                "INSERT INTO messages (
                    uuid, session_id, parent_uuid, type, role,
                    is_sidechain, agent_id, model, content_text, content_json,
                    has_tool_use, has_thinking, tool_names,
                    input_tokens, output_tokens, stop_reason,
                    timestamp, line_number
                ) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,?18)",
            )?;

            for msg in &result.messages {
                let tool_names_str = if msg.tool_names.is_empty() {
                    None
                } else {
                    Some(msg.tool_names.join(","))
                };
                stmt.execute(params![
                    msg.uuid,
                    sf.session_id,
                    msg.parent_uuid,
                    msg.msg_type,
                    msg.role,
                    msg.is_sidechain as i64,
                    msg.agent_id,
                    msg.model,
                    msg.content_text,
                    msg.content_json,
                    msg.has_tool_use as i64,
                    msg.has_thinking as i64,
                    tool_names_str,
                    msg.input_tokens,
                    msg.output_tokens,
                    msg.stop_reason,
                    msg.timestamp,
                    msg.line_number,
                ])?;
            }

            Ok(())
        })();

        match insert_result {
            Ok(()) => {
                total_messages += result.messages.len() as i64;
                tx.commit().map_err(|e| e.to_string())?;
            }
            Err(e) => {
                let _ = tx.rollback();
                log::error!("Error writing session {}: {}", sf.session_id, e);
            }
        }
    }

    // Phase 5: Parse subagents
    emit_progress(app, "Parsing subagents", 0, 1);
    {
        let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
        for sf in &session_files {
            let subagents_dir = projects_dir
                .join(&sf.project_encoded)
                .join(&sf.session_id)
                .join("subagents");
            if !subagents_dir.exists() {
                continue;
            }

            if let Ok(entries) = fs::read_dir(&subagents_dir) {
                for entry in entries.filter_map(Result::ok) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !name.ends_with(".meta.json") {
                        continue;
                    }

                    let agent_id = name
                        .trim_end_matches(".meta.json")
                        .trim_start_matches("agent-")
                        .to_string();
                    let meta_path = entry.path();
                    let jsonl_path = subagents_dir.join(name.replace(".meta.json", ".jsonl"));

                    if let Ok(content) = fs::read_to_string(&meta_path) {
                        if let Ok(meta) = serde_json::from_str::<SubagentMeta>(&content) {
                            let line_count: i64 = if jsonl_path.exists() {
                                fs::read_to_string(&jsonl_path)
                                    .map(|c| {
                                        c.lines().filter(|l| !l.trim().is_empty()).count() as i64
                                    })
                                    .unwrap_or(0)
                            } else {
                                0
                            };

                            let _ = tx.execute(
                                "INSERT OR REPLACE INTO subagents (id, session_id, agent_type, description, file_path, message_count)
                                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                                params![
                                    agent_id,
                                    sf.session_id,
                                    meta.agent_type,
                                    meta.description,
                                    jsonl_path.to_string_lossy().to_string(),
                                    line_count,
                                ],
                            );
                        }
                    }
                }
            }
        }
        tx.commit().map_err(|e| e.to_string())?;
    }

    // Phase 6: Check active sessions
    emit_progress(app, "Checking active sessions", 0, 1);
    {
        let tx = conn.unchecked_transaction().map_err(|e| e.to_string())?;
        tx.execute("UPDATE sessions SET is_active = 0", [])
            .map_err(|e| e.to_string())?;

        let sessions_dir = claude_dir.join("sessions");
        if sessions_dir.exists() {
            if let Ok(entries) = fs::read_dir(&sessions_dir) {
                for entry in entries.filter_map(Result::ok) {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !name.ends_with(".json") {
                        continue;
                    }
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        if let Ok(data) = serde_json::from_str::<ActiveSessionFile>(&content) {
                            if let Some(sid) = data.session_id {
                                let _ = tx.execute(
                                    "UPDATE sessions SET is_active = 1 WHERE id = ?1",
                                    params![sid],
                                );
                            }
                        }
                    }
                }
            }
        }
        tx.commit().map_err(|e| e.to_string())?;
    }

    // Phase 7: Update project aggregates
    emit_progress(app, "Updating project stats", 0, 1);
    conn.execute_batch(
        "UPDATE projects SET
            session_count = (SELECT COUNT(*) FROM sessions WHERE project_id = projects.id),
            total_tokens = (SELECT COALESCE(SUM(total_input_tokens + total_output_tokens), 0) FROM sessions WHERE project_id = projects.id),
            last_activity_at = (SELECT MAX(modified_at) FROM sessions WHERE project_id = projects.id)"
    ).map_err(|e| e.to_string())?;

    // Save sync state
    conn.execute(
        "INSERT INTO sync_state (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params!["last_sync_at", chrono::Utc::now().to_rfc3339()],
    )
    .map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO sync_state (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params!["session_count", total_sessions.to_string()],
    )
    .map_err(|e| e.to_string())?;

    emit_progress(app, "Done", total_sessions, total_sessions);

    Ok((total_sessions, total_messages))
}

use std::fs;
use std::io::{BufRead, BufReader, Write};

use rusqlite::params;
use tauri::State;

use crate::db::DbPool;
use crate::types::api::{ReplaceAllParams, ReplaceOneParams, ReplaceResult};

/// Replace all occurrences of `needle` in `haystack`, respecting case sensitivity.
/// Returns (new_string, replacement_count).
pub(crate) fn replace_in_string(
    haystack: &str,
    needle: &str,
    replacement: &str,
    case_sensitive: bool,
) -> (String, usize) {
    if needle.is_empty() {
        return (haystack.to_string(), 0);
    }
    if case_sensitive {
        let count = haystack.matches(needle).count();
        (haystack.replace(needle, replacement), count)
    } else {
        let lower_haystack = haystack.to_lowercase();
        let lower_needle = needle.to_lowercase();
        let mut result = String::with_capacity(haystack.len());
        let mut count = 0usize;
        let mut start = 0;
        while let Some(pos) = lower_haystack[start..].find(&lower_needle) {
            result.push_str(&haystack[start..start + pos]);
            result.push_str(replacement);
            start += pos + needle.len();
            count += 1;
        }
        result.push_str(&haystack[start..]);
        (result, count)
    }
}

/// Replace only the Nth occurrence (0-based) of `needle` in `haystack`.
/// Returns (new_string, true if replaced).
fn replace_nth_in_string(
    haystack: &str,
    needle: &str,
    replacement: &str,
    case_sensitive: bool,
    target_index: usize,
) -> (String, bool) {
    if needle.is_empty() {
        return (haystack.to_string(), false);
    }
    let lower_haystack = if case_sensitive {
        haystack.to_string()
    } else {
        haystack.to_lowercase()
    };
    let lower_needle = if case_sensitive {
        needle.to_string()
    } else {
        needle.to_lowercase()
    };

    let mut result = String::with_capacity(haystack.len());
    let mut count = 0usize;
    let mut start = 0;
    while let Some(pos) = lower_haystack[start..].find(&lower_needle) {
        if count == target_index {
            result.push_str(&haystack[start..start + pos]);
            result.push_str(replacement);
            start += pos + needle.len();
            // Append the rest unchanged
            result.push_str(&haystack[start..]);
            return (result, true);
        }
        result.push_str(&haystack[start..start + pos + needle.len()]);
        start += pos + needle.len();
        count += 1;
    }
    result.push_str(&haystack[start..]);
    (result, false)
}

/// Keys to skip when recursively replacing in JSON values.
/// These are structural/identity fields that should not be modified.
const SKIP_KEYS: &[&str] = &[
    "type",
    "role",
    "id",
    "tool_use_id",
    "name",
    "signature",
    "is_error",
    "stop_reason",
    "stop_sequence",
    "model",
];

/// Recursively replace `needle` in all string values within a JSON value,
/// skipping keys listed in `SKIP_KEYS`.
fn replace_all_in_json(
    val: &mut serde_json::Value,
    needle: &str,
    replacement: &str,
    case_sensitive: bool,
) -> usize {
    match val {
        serde_json::Value::String(s) => {
            let (new_s, count) = replace_in_string(s, needle, replacement, case_sensitive);
            if count > 0 {
                *s = new_s;
            }
            count
        }
        serde_json::Value::Array(arr) => arr
            .iter_mut()
            .map(|v| replace_all_in_json(v, needle, replacement, case_sensitive))
            .sum(),
        serde_json::Value::Object(map) => map
            .iter_mut()
            .filter(|(k, _)| !SKIP_KEYS.contains(&k.as_str()))
            .map(|(_, v)| replace_all_in_json(v, needle, replacement, case_sensitive))
            .sum(),
        _ => 0,
    }
}

/// Replace text in a JSONL message value's content fields.
/// Recursively replaces in all string values within message content,
/// including text blocks, tool inputs, thinking blocks, and tool results.
fn replace_all_in_value(
    val: &mut serde_json::Value,
    needle: &str,
    replacement: &str,
    case_sensitive: bool,
) -> usize {
    let msg_type = match val.get("type").and_then(|t| t.as_str()) {
        Some(t) => t.to_string(),
        None => return 0,
    };

    let mut total = 0;

    // Replace in cwd field
    if let Some(cwd) = val.get_mut("cwd") {
        total += replace_all_in_json(cwd, needle, replacement, case_sensitive);
    }

    match msg_type.as_str() {
        "user" | "assistant" => {
            if let Some(content) = val.get_mut("message").and_then(|m| m.get_mut("content")) {
                total += replace_all_in_json(content, needle, replacement, case_sensitive);
            }
        }
        _ => {}
    }

    total
}

/// Replace only the Nth occurrence across all string values in a JSON tree.
/// Returns true if the replacement was made.
fn replace_nth_in_json(
    val: &mut serde_json::Value,
    needle: &str,
    replacement: &str,
    case_sensitive: bool,
    target: usize,
    counter: &mut usize,
) -> bool {
    match val {
        serde_json::Value::String(s) => {
            let n = count_occurrences(s, needle, case_sensitive);
            if target >= *counter && target < *counter + n {
                let local = target - *counter;
                let (new_s, did) =
                    replace_nth_in_string(s, needle, replacement, case_sensitive, local);
                if did {
                    *s = new_s;
                    return true;
                }
            }
            *counter += n;
            false
        }
        serde_json::Value::Array(arr) => {
            for v in arr.iter_mut() {
                if replace_nth_in_json(v, needle, replacement, case_sensitive, target, counter) {
                    return true;
                }
            }
            false
        }
        serde_json::Value::Object(map) => {
            for (k, v) in map.iter_mut() {
                if SKIP_KEYS.contains(&k.as_str()) {
                    continue;
                }
                if replace_nth_in_json(v, needle, replacement, case_sensitive, target, counter) {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

/// Replace only a specific occurrence in a JSONL line's content.
/// `occurrence_index` is the 0-based index across all text fields in the message.
fn replace_one_in_value(
    val: &mut serde_json::Value,
    needle: &str,
    replacement: &str,
    case_sensitive: bool,
    occurrence_index: usize,
) -> bool {
    let msg_type = match val.get("type").and_then(|t| t.as_str()) {
        Some(t) => t.to_string(),
        None => return false,
    };

    // Count occurrences in cwd first
    let mut counter = 0usize;
    if let Some(cwd) = val.get_mut("cwd") {
        if replace_nth_in_json(
            cwd,
            needle,
            replacement,
            case_sensitive,
            occurrence_index,
            &mut counter,
        ) {
            return true;
        }
    }

    match msg_type.as_str() {
        "user" | "assistant" => {
            if let Some(content) = val.get_mut("message").and_then(|m| m.get_mut("content")) {
                if replace_nth_in_json(
                    content,
                    needle,
                    replacement,
                    case_sensitive,
                    occurrence_index,
                    &mut counter,
                ) {
                    return true;
                }
            }
        }
        _ => {}
    }

    false
}

fn count_occurrences(haystack: &str, needle: &str, case_sensitive: bool) -> usize {
    if needle.is_empty() {
        return 0;
    }
    if case_sensitive {
        haystack.matches(needle).count()
    } else {
        let lower = haystack.to_lowercase();
        let lower_needle = needle.to_lowercase();
        let mut count = 0;
        let mut start = 0;
        while let Some(pos) = lower[start..].find(&lower_needle) {
            count += 1;
            start += pos + lower_needle.len();
        }
        count
    }
}

/// Extract plain text content from a parsed JSONL value, mirroring the sync parser logic.
fn extract_content_text(val: &serde_json::Value) -> Option<String> {
    let msg_type = val.get("type").and_then(|t| t.as_str())?;
    match msg_type {
        "user" => {
            let content = val.get("message")?.get("content")?;
            if let Some(s) = content.as_str() {
                return Some(s.to_string());
            }
            if let Some(arr) = content.as_array() {
                let mut parts = Vec::new();
                for block in arr {
                    let bt = block.get("type").and_then(|t| t.as_str()).unwrap_or("");
                    match bt {
                        "text" => {
                            if let Some(text) = block.get("text").and_then(|t| t.as_str()) {
                                parts.push(text.to_string());
                            }
                        }
                        "tool_result" => {
                            if let Some(c) = block.get("content").and_then(|c| c.as_str()) {
                                if c.len() < 1000 {
                                    parts.push(c.to_string());
                                }
                            }
                        }
                        _ => {}
                    }
                }
                return Some(parts.join("\n"));
            }
            None
        }
        "assistant" => {
            let content = val.get("message")?.get("content")?;
            if let Some(arr) = content.as_array() {
                let parts: Vec<String> = arr
                    .iter()
                    .filter(|b| b.get("type").and_then(|t| t.as_str()) == Some("text"))
                    .filter_map(|b| b.get("text").and_then(|t| t.as_str()).map(String::from))
                    .collect();
                return Some(parts.join("\n"));
            }
            None
        }
        _ => None,
    }
}

/// Extract content_json for assistant messages (the content array serialized).
fn extract_content_json(val: &serde_json::Value) -> Option<String> {
    let msg_type = val.get("type").and_then(|t| t.as_str())?;
    if msg_type == "assistant" {
        let content = val.get("message")?.get("content")?;
        serde_json::to_string(content).ok()
    } else {
        None
    }
}

pub(crate) fn get_file_path(pool: &DbPool, session_id: &str) -> Result<String, String> {
    let conn = pool.get().map_err(|e| e.to_string())?;
    let file_path: String = conn
        .query_row(
            "SELECT file_path FROM sessions WHERE id = ?1",
            params![session_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("Session not found: {}", e))?;

    if !std::path::Path::new(&file_path).exists() {
        return Err("Session file not found on disk".to_string());
    }
    Ok(file_path)
}

#[tauri::command]
pub fn replace_in_session(
    params: ReplaceAllParams,
    pool: State<'_, DbPool>,
) -> Result<ReplaceResult, String> {
    if params.search_term.is_empty() {
        return Err("Search term cannot be empty".to_string());
    }

    let file_path = get_file_path(&pool, &params.session_id)?;
    let case_sensitive = params.case_sensitive.unwrap_or(false);

    log::info!(
        "replace_in_session: session={}, search='{}', replace='{}', case_sensitive={}",
        params.session_id,
        params.search_term,
        params.replace_term,
        case_sensitive
    );

    let src_file = fs::File::open(&file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(src_file);

    let path = std::path::Path::new(&file_path);
    let parent = path.parent().unwrap_or(path);
    let tmp_path = parent.join(format!(
        ".{}.replace.tmp",
        path.file_name().unwrap().to_string_lossy()
    ));

    let mut total_replaced = 0usize;
    // Collect (line_number, new_content_text, new_content_json) for DB updates
    let mut db_updates: Vec<(i64, Option<String>, Option<String>)> = Vec::new();

    {
        let mut writer = fs::File::create(&tmp_path)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        for (line_number, line_result) in (1_i64..).zip(reader.lines()) {
            let line = line_result.map_err(|e| format!("Failed to read line: {}", e))?;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                writeln!(writer).map_err(|e| format!("Failed to write: {}", e))?;
                continue;
            }

            match serde_json::from_str::<serde_json::Value>(trimmed) {
                Ok(mut val) => {
                    let count = replace_all_in_value(
                        &mut val,
                        &params.search_term,
                        &params.replace_term,
                        case_sensitive,
                    );
                    if count > 0 {
                        total_replaced += count;
                        let text = extract_content_text(&val);
                        let json = extract_content_json(&val);
                        db_updates.push((line_number, text, json));
                    }
                    let rewritten = serde_json::to_string(&val)
                        .map_err(|e| format!("Failed to serialize: {}", e))?;
                    writeln!(writer, "{}", rewritten)
                        .map_err(|e| format!("Failed to write: {}", e))?;
                }
                Err(_) => {
                    writeln!(writer, "{}", trimmed)
                        .map_err(|e| format!("Failed to write: {}", e))?;
                }
            }
        }

        writer
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
    }

    log::info!("replace_in_session: total_replaced={}", total_replaced);

    // Atomic rename
    fs::rename(&tmp_path, &file_path).map_err(|e| format!("Failed to rename temp file: {}", e))?;

    // Update DB
    if !db_updates.is_empty() {
        let conn = pool.get().map_err(|e| e.to_string())?;
        for (line_num, text, json) in &db_updates {
            if let Some(text) = text {
                conn.execute(
                    "UPDATE messages SET content_text = ?1, content_json = COALESCE(?2, content_json)
                     WHERE session_id = ?3 AND line_number = ?4",
                    params![text, json, params.session_id, line_num],
                )
                .map_err(|e| format!("Failed to update message: {}", e))?;
            }
        }
    }

    Ok(ReplaceResult {
        replaced_count: total_replaced,
    })
}

#[tauri::command]
pub fn replace_one_in_session(
    params: ReplaceOneParams,
    pool: State<'_, DbPool>,
) -> Result<ReplaceResult, String> {
    if params.search_term.is_empty() {
        return Err("Search term cannot be empty".to_string());
    }

    let file_path = get_file_path(&pool, &params.session_id)?;
    let case_sensitive = params.case_sensitive.unwrap_or(false);

    let src_file = fs::File::open(&file_path).map_err(|e| format!("Failed to open file: {}", e))?;
    let reader = BufReader::new(src_file);

    let path = std::path::Path::new(&file_path);
    let parent = path.parent().unwrap_or(path);
    let tmp_path = parent.join(format!(
        ".{}.replace.tmp",
        path.file_name().unwrap().to_string_lossy()
    ));

    let mut replaced = false;

    {
        let mut writer = fs::File::create(&tmp_path)
            .map_err(|e| format!("Failed to create temp file: {}", e))?;

        for (line_number, line_result) in (1_i64..).zip(reader.lines()) {
            let line = line_result.map_err(|e| format!("Failed to read line: {}", e))?;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                writeln!(writer).map_err(|e| format!("Failed to write: {}", e))?;
                continue;
            }

            if line_number == params.line_number {
                match serde_json::from_str::<serde_json::Value>(trimmed) {
                    Ok(mut val) => {
                        let did = replace_one_in_value(
                            &mut val,
                            &params.search_term,
                            &params.replace_term,
                            case_sensitive,
                            params.occurrence_index,
                        );
                        if did {
                            replaced = true;
                            // Update DB for this line
                            let text = extract_content_text(&val);
                            let json = extract_content_json(&val);
                            let conn = pool.get().map_err(|e| e.to_string())?;
                            if let Some(text) = &text {
                                conn.execute(
                                    "UPDATE messages SET content_text = ?1, content_json = COALESCE(?2, content_json)
                                     WHERE session_id = ?3 AND line_number = ?4",
                                    params![text, json, params.session_id, line_number],
                                )
                                .map_err(|e| format!("Failed to update message: {}", e))?;
                            }
                        }
                        let rewritten = serde_json::to_string(&val)
                            .map_err(|e| format!("Failed to serialize: {}", e))?;
                        writeln!(writer, "{}", rewritten)
                            .map_err(|e| format!("Failed to write: {}", e))?;
                    }
                    Err(_) => {
                        writeln!(writer, "{}", trimmed)
                            .map_err(|e| format!("Failed to write: {}", e))?;
                    }
                }
            } else {
                writeln!(writer, "{}", trimmed).map_err(|e| format!("Failed to write: {}", e))?;
            }
        }

        writer
            .flush()
            .map_err(|e| format!("Failed to flush: {}", e))?;
    }

    // Atomic rename
    fs::rename(&tmp_path, &file_path).map_err(|e| format!("Failed to rename temp file: {}", e))?;

    Ok(ReplaceResult {
        replaced_count: if replaced { 1 } else { 0 },
    })
}

use tauri::{AppHandle, State};

use crate::db::DbPool;
use crate::sync::engine::full_sync;
use crate::types::api::SyncResult;

#[tauri::command]
pub async fn trigger_sync(
    pool: State<'_, DbPool>,
    app: AppHandle,
) -> Result<SyncResult, String> {
    let pool = pool.inner().clone();
    let result = tokio::task::spawn_blocking(move || full_sync(&pool, &app))
        .await
        .map_err(|e| e.to_string())?;

    match result {
        Ok((sessions, messages)) => Ok(SyncResult {
            ok: true,
            sessions,
            messages,
        }),
        Err(e) => Err(e),
    }
}

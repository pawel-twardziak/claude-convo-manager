pub mod schema;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags;
use std::path::PathBuf;

pub type DbPool = Pool<SqliteConnectionManager>;

pub fn create_pool(db_path: &PathBuf) -> Result<DbPool, Box<dyn std::error::Error>> {
    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let manager = SqliteConnectionManager::file(db_path)
        .with_flags(
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )
        .with_init(|conn| {
            conn.execute_batch(
                "PRAGMA journal_mode = WAL;
                 PRAGMA foreign_keys = ON;
                 PRAGMA busy_timeout = 5000;",
            )?;
            Ok(())
        });

    let pool = Pool::builder().max_size(8).build(manager)?;

    // Initialize schema on first connection
    {
        let conn = pool.get()?;
        schema::initialize_schema(&conn)?;
    }

    Ok(pool)
}

pub fn get_db_path(app_handle: &tauri::AppHandle) -> PathBuf {
    use tauri::Manager;
    let data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("failed to get app data dir");
    data_dir.join("ccm.db")
}

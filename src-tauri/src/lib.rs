mod commands;
mod db;
mod sync;
mod types;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_process::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            // Initialize database
            let db_path = db::get_db_path(app.handle());
            let pool = db::create_pool(&db_path).expect("Failed to create database pool");
            app.manage(pool);

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::sessions::get_sessions,
            commands::sessions::get_session,
            commands::sessions::get_filter_options,
            commands::messages::get_session_messages,
            commands::search::search_messages,
            commands::analytics::get_dashboard_stats,
            commands::analytics::get_token_usage_over_time,
            commands::analytics::get_project_breakdown,
            commands::analytics::get_activity_data,
            commands::projects::get_projects,
            commands::sync::trigger_sync,
            commands::rename::rename_session,
            commands::clone::clone_session,
            commands::ide::detect_available_apps,
            commands::ide::open_in_app,
            commands::ide::open_terminal,
            commands::replace::replace_in_session,
            commands::replace::replace_one_in_session,
            commands::delete::delete_session,
            commands::delete_message::delete_last_message,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

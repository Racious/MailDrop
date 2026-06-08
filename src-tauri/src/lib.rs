mod api;
mod commands;
mod db;
mod models;
mod smtp;
mod tray;

use db::{connection, migrations, DbPool};
use std::sync::{
    atomic::AtomicBool,
    Arc,
};
use tauri::Manager;

pub struct AppState {
    pub db_pool: DbPool,
    pub smtp_running: Arc<AtomicBool>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::Builder::new().callback(|app, _argv, _cwd| {
            tray::show_main_window(app);
        }).build())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // ── Database ──────────────────────────────────────────────────
            let data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&data_dir)?;
            let db_path = data_dir.join("maildrop.db");

            let pool = connection::create_pool(&db_path)
                .expect("failed to create DB pool");
            migrations::run(&pool).expect("DB migration failed");

            // ── SMTP port ─────────────────────────────────────────────────
            let smtp_port = db::repository::get_config_value(&pool, "smtp_port")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1025u16);

            // ── System tray ───────────────────────────────────────────────
            tray::setup(app)?;

            // ── Close-to-tray ─────────────────────────────────────────────
            if let Some(window) = app.get_webview_window("main") {
                let win = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        api.prevent_close();
                        let _ = win.hide();
                    }
                });
            }

            // ── SMTP server (separate OS thread + own Tokio runtime) ───────
            let smtp_running = Arc::new(AtomicBool::new(false));
            let smtp_pool = pool.clone();
            let smtp_handle = app.handle().clone();
            let smtp_running_flag = smtp_running.clone();
            std::thread::spawn(move || {
                smtp::server::start(smtp_port, smtp_pool, smtp_handle, smtp_running_flag);
            });

            let api_pool = pool.clone();
            std::thread::spawn(move || {
                let rt = tokio::runtime::Runtime::new().expect("API tokio runtime");
                rt.block_on(api::start(api_pool));
            });

            // ── Register AppState ─────────────────────────────────────────
            app.manage(AppState { db_pool: pool, smtp_running });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::mail::list_mails,
            commands::mail::search_mails,
            commands::mail::get_mail,
            commands::mail::delete_mail,
            commands::mail::clear_mails,
            commands::mail::get_mail_count,
            commands::mail::mark_as_read,
            commands::mail::get_unread_count,
            commands::mail::list_attachments,
            commands::mail::get_attachment_content,
            commands::mail::list_smtp_sessions,
            commands::config::get_config,
            commands::config::save_config,
            commands::config::get_smtp_status,
            commands::config::restart_app,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use crate::db::repository;
use crate::models::AppConfig;
use crate::AppState;
use std::sync::atomic::Ordering;
use tauri::{AppHandle, Emitter, State};

#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let pool = state.db_pool.clone();
    tokio::task::spawn_blocking(move || repository::get_config(&pool))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_config(
    state: State<'_, AppState>,
    app_handle: AppHandle,
    config: AppConfig,
) -> Result<(), String> {
    let pool = state.db_pool.clone();
    let cfg = config.clone();
    tokio::task::spawn_blocking(move || {
        repository::set_config_value(&pool, "smtp_port", &cfg.smtp_port.to_string())?;
        repository::set_config_value(&pool, "theme", &cfg.theme)?;
        repository::set_config_value(&pool, "max_mails", &cfg.max_mails.to_string())?;
        Ok::<_, Box<dyn std::error::Error + Send + Sync>>(())
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?;

    let _ = app_handle.emit("config:saved", &config);
    Ok(())
}

#[tauri::command]
pub fn get_smtp_status(state: State<'_, AppState>) -> bool {
    state.smtp_running.load(Ordering::SeqCst)
}

#[tauri::command]
pub fn restart_app(app_handle: AppHandle) {
    app_handle.restart();
}

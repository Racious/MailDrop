use crate::db::repository;
use crate::models::{Mail, MailSummary};
use crate::AppState;
use tauri::State;

#[tauri::command]
pub async fn list_mails(
    state: State<'_, AppState>,
    offset: u32,
    limit: u32,
) -> Result<Vec<MailSummary>, String> {
    let pool = state.db_pool.clone();
    tokio::task::spawn_blocking(move || repository::list_mails(&pool, offset, limit))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_mail(state: State<'_, AppState>, id: String) -> Result<Mail, String> {
    let pool = state.db_pool.clone();
    let id2 = id.clone();
    tokio::task::spawn_blocking(move || repository::get_mail(&pool, &id2))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("mail {id} not found"))
}

#[tauri::command]
pub async fn delete_mail(state: State<'_, AppState>, id: String) -> Result<String, String> {
    let pool = state.db_pool.clone();
    let id2 = id.clone();
    tokio::task::spawn_blocking(move || repository::delete_mail(&pool, &id2))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())?;
    Ok(id)
}

#[tauri::command]
pub async fn clear_mails(state: State<'_, AppState>) -> Result<usize, String> {
    let pool = state.db_pool.clone();
    tokio::task::spawn_blocking(move || repository::clear_mails(&pool))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_mail_count(state: State<'_, AppState>) -> Result<u32, String> {
    let pool = state.db_pool.clone();
    tokio::task::spawn_blocking(move || repository::get_mail_count(&pool))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

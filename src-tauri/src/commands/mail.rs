use crate::db::repository;
use crate::models::{Mail, MailAttachment, MailSearchResult, MailSummary, SmtpSessionLog};
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
pub async fn search_mails(
    state: State<'_, AppState>,
    query: String,
    field: String,
    unread_only: bool,
    has_attachments: bool,
    offset: u32,
    limit: u32,
) -> Result<MailSearchResult, String> {
    let pool = state.db_pool.clone();
    tokio::task::spawn_blocking(move || {
        repository::search_mails(
            &pool,
            &query,
            &field,
            unread_only,
            has_attachments,
            offset,
            limit,
        )
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())
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

#[tauri::command]
pub async fn mark_as_read(state: State<'_, AppState>, id: String) -> Result<(), String> {
    let pool = state.db_pool.clone();
    tokio::task::spawn_blocking(move || repository::mark_as_read(&pool, &id))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_unread_count(state: State<'_, AppState>) -> Result<u32, String> {
    let pool = state.db_pool.clone();
    tokio::task::spawn_blocking(move || repository::get_unread_count(&pool))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_attachments(
    state: State<'_, AppState>,
    mail_id: String,
) -> Result<Vec<MailAttachment>, String> {
    let pool = state.db_pool.clone();
    tokio::task::spawn_blocking(move || repository::list_attachments(&pool, &mail_id))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_attachment_content(
    state: State<'_, AppState>,
    mail_id: String,
    attachment_id: String,
) -> Result<Vec<u8>, String> {
    let pool = state.db_pool.clone();
    tokio::task::spawn_blocking(move || {
        repository::get_attachment_content(&pool, &mail_id, &attachment_id)
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?
    .map(|(_, content)| content)
    .ok_or_else(|| "attachment not found".to_string())
}

#[tauri::command]
pub async fn list_smtp_sessions(
    state: State<'_, AppState>,
    limit: u32,
) -> Result<Vec<SmtpSessionLog>, String> {
    let pool = state.db_pool.clone();
    tokio::task::spawn_blocking(move || repository::list_smtp_sessions(&pool, limit))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}

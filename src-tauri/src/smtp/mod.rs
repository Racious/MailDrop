pub mod parser;
pub mod server;
pub mod session;

use crate::db::{repository, DbPool};
use crate::models::{Mail, MailSummary};
use session::SmtpMessage;
use tauri::{AppHandle, Emitter, Manager, UserAttentionType};
use tauri_plugin_notification::NotificationExt;

pub async fn handle_message(msg: SmtpMessage, pool: DbPool, app_handle: AppHandle) {
    let parsed = parser::parse(&msg.data);

    let mail = Mail {
        id: uuid::Uuid::new_v4().to_string(),
        message_id: parsed.message_id,
        from_addr: if parsed.from_addr.is_empty() {
            msg.from.clone()
        } else {
            parsed.from_addr
        },
        from_name: parsed.from_name,
        to_addrs: if parsed.to_addrs.is_empty() {
            msg.to.clone()
        } else {
            parsed.to_addrs
        },
        subject: parsed.subject,
        text_body: parsed.text_body,
        html_body: parsed.html_body.clone(),
        raw_mime: String::from_utf8_lossy(&msg.data).into_owned(),
        size_bytes: msg.data.len() as u32,
        received_at: chrono::Utc::now().to_rfc3339(),
        has_html: parsed.html_body.is_some(),
    };

    let summary = MailSummary {
        id: mail.id.clone(),
        from_addr: mail.from_addr.clone(),
        from_name: mail.from_name.clone(),
        subject: mail.subject.clone(),
        received_at: mail.received_at.clone(),
        size_bytes: mail.size_bytes,
        has_html: mail.has_html,
        is_read: false,
    };

    let pool_save = pool.clone();
    let mail_clone = mail.clone();
    if let Err(e) = tokio::task::spawn_blocking(move || repository::insert_mail(&pool_save, &mail_clone)).await {
        eprintln!("[smtp] failed to save mail: {e}");
        return;
    }

    // Enforce max_mails limit
    let pool_trim = pool.clone();
    tokio::task::spawn_blocking(move || {
        if let Ok(max) = repository::get_config_value(&pool_trim, "max_mails")
            .map(|v| v.parse::<usize>().unwrap_or(500))
        {
            let _ = repository::enforce_max_mails(&pool_trim, max);
        }
    });

    let _ = app_handle.emit("mail:received", &summary);

    let notify_enabled = repository::get_config_value(&pool, "enable_notifications")
        .ok()
        .and_then(|v| v.parse::<bool>().ok())
        .unwrap_or(true);

    if notify_enabled {
        let title = if mail.subject.is_empty() { "(無主旨)".to_string() } else { mail.subject.clone() };
        let body = format!("來自：{}", mail.from_addr);
        let _ = app_handle.notification()
            .builder()
            .title(&title)
            .body(&body)
            .show();

        // 視窗未在前景時，工具列圖示持續閃爍直到使用者點開
        if let Some(window) = app_handle.get_webview_window("main") {
            let focused = window.is_focused().unwrap_or(true);
            if !focused {
                let _ = window.request_user_attention(Some(UserAttentionType::Critical));
            }
        }
    }
}

use crate::db::DbPool;
use crate::db::repository;
use crate::models::SmtpSessionLog;
use crate::smtp;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{AppHandle, Emitter};
use tokio::net::TcpListener;

pub fn start(port: u16, pool: DbPool, app_handle: AppHandle, running: Arc<AtomicBool>) {
    let rt = tokio::runtime::Runtime::new().expect("SMTP tokio runtime");
    rt.block_on(async move {
        let addr = format!("127.0.0.1:{port}");
        let listener = match TcpListener::bind(&addr).await {
            Ok(l) => {
                running.store(true, Ordering::SeqCst);
                println!("[smtp] listening on {addr}");
                let _ = app_handle.emit("smtp:started", port);
                l
            }
            Err(e) => {
                running.store(false, Ordering::SeqCst);
                eprintln!("[smtp] failed to bind {addr}: {e}");
                let _ = app_handle.emit(
                    "smtp:error",
                    format!("Port {port} 已被其他程式佔用，請至 Settings 更換 Port 後重啟"),
                );
                return;
            }
        };

        loop {
            match listener.accept().await {
                Ok((stream, remote_addr)) => {
                    let pool = pool.clone();
                    let handle = app_handle.clone();
                    tokio::spawn(async move {
                        let session_id = uuid::Uuid::new_v4().to_string();
                        let started_at = chrono::Utc::now().to_rfc3339();
                        match smtp::session::handle(stream).await {
                            Ok(result) => {
                                let mut mail_id = None;
                                if let Some(msg) = result.message {
                                    let id = smtp::handle_message(msg, pool.clone(), handle).await;
                                    mail_id = Some(id);
                                }
                                let session = SmtpSessionLog {
                                    id: session_id,
                                    mail_id,
                                    remote_addr: remote_addr.to_string(),
                                    started_at,
                                    ended_at: chrono::Utc::now().to_rfc3339(),
                                    transcript: result.transcript,
                                    error: result.error,
                                };
                                let pool_log = pool.clone();
                                let _ = tokio::task::spawn_blocking(move || {
                                    repository::insert_smtp_session(&pool_log, &session)
                                })
                                .await;
                            }
                            Err(e) => eprintln!("[smtp] session error: {e}"),
                        }
                    });
                }
                Err(e) => eprintln!("[smtp] accept error: {e}"),
            }
        }
    });
}

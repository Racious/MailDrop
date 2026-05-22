use crate::db::DbPool;
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
                Ok((stream, _)) => {
                    let pool = pool.clone();
                    let handle = app_handle.clone();
                    tokio::spawn(async move {
                        match smtp::session::handle(stream).await {
                            Ok(Some(msg)) => smtp::handle_message(msg, pool, handle).await,
                            Ok(None) => {}
                            Err(e) => eprintln!("[smtp] session error: {e}"),
                        }
                    });
                }
                Err(e) => eprintln!("[smtp] accept error: {e}"),
            }
        }
    });
}

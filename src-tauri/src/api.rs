use crate::db::{repository, DbPool};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const API_ADDR: &str = "127.0.0.1:8025";

pub async fn start(pool: DbPool) {
    let listener = match TcpListener::bind(API_ADDR).await {
        Ok(listener) => listener,
        Err(e) => {
            eprintln!("[api] failed to bind {API_ADDR}: {e}");
            return;
        }
    };
    println!("[api] listening on http://{API_ADDR}");

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let pool = pool.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(stream, pool).await {
                        eprintln!("[api] request error: {e}");
                    }
                });
            }
            Err(e) => eprintln!("[api] accept error: {e}"),
        }
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    pool: DbPool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = vec![0u8; 8192];
    let n = stream.read(&mut buffer).await?;
    if n == 0 {
        return Ok(());
    }

    let request = String::from_utf8_lossy(&buffer[..n]);
    let Some(first_line) = request.lines().next() else {
        write_response(&mut stream, 400, "text/plain", b"Bad Request").await?;
        return Ok(());
    };
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or_default();
    let target = parts.next().unwrap_or_default();
    let (path, query) = split_target(target);

    match (method, path.as_slice()) {
        ("GET", ["api", "messages"]) => {
            let limit = query_param(query, "limit")
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(100)
                .min(500);
            let pool_query = pool.clone();
            let has_search = query_param(query, "query").is_some()
                || query_param(query, "field").is_some()
                || query_param(query, "unreadOnly").is_some()
                || query_param(query, "hasAttachments").is_some();
            if has_search {
                let search_query = query_param(query, "query").unwrap_or_default().to_string();
                let field = query_param(query, "field").unwrap_or("all").to_string();
                let unread_only = query_bool(query, "unreadOnly");
                let has_attachments = query_bool(query, "hasAttachments");
                let result = tokio::task::spawn_blocking(move || {
                    repository::search_mails(
                        &pool_query,
                        &search_query,
                        &field,
                        unread_only,
                        has_attachments,
                        0,
                        limit,
                    )
                })
                .await??;
                write_json(&mut stream, &result).await?;
            } else {
                let mails = tokio::task::spawn_blocking(move || repository::list_mails(&pool_query, 0, limit))
                    .await??;
                write_json(&mut stream, &mails).await?;
            }
        }
        ("GET", ["api", "messages", id]) => {
            let id = id.to_string();
            let pool_query = pool.clone();
            let mail = tokio::task::spawn_blocking(move || repository::get_mail(&pool_query, &id))
                .await??;
            match mail {
                Some(mail) => write_json(&mut stream, &mail).await?,
                None => write_response(&mut stream, 404, "text/plain", b"Not Found").await?,
            }
        }
        ("GET", ["api", "messages", mail_id, "attachments", attachment_id]) => {
            let mail_id = mail_id.to_string();
            let attachment_id = attachment_id.to_string();
            let pool_query = pool.clone();
            let attachment = tokio::task::spawn_blocking(move || {
                repository::get_attachment_content(&pool_query, &mail_id, &attachment_id)
            })
            .await??;
            match attachment {
                Some((meta, content)) => {
                    write_download(&mut stream, &meta.filename, &meta.content_type, &content).await?
                }
                None => write_response(&mut stream, 404, "text/plain", b"Not Found").await?,
            }
        }
        ("GET", ["api", "sessions"]) => {
            let limit = query_param(query, "limit")
                .and_then(|v| v.parse::<u32>().ok())
                .unwrap_or(100)
                .min(500);
            let pool_query = pool.clone();
            let sessions = tokio::task::spawn_blocking(move || repository::list_smtp_sessions(&pool_query, limit))
                .await??;
            write_json(&mut stream, &sessions).await?;
        }
        ("DELETE", ["api", "messages"]) => {
            let pool_query = pool.clone();
            let deleted = tokio::task::spawn_blocking(move || repository::clear_mails(&pool_query))
                .await??;
            write_json(&mut stream, &serde_json::json!({ "deleted": deleted })).await?;
        }
        _ => write_response(&mut stream, 404, "text/plain", b"Not Found").await?,
    }

    Ok(())
}

fn split_target(target: &str) -> (Vec<&str>, Option<&str>) {
    let (path, query) = target.split_once('?').map_or((target, None), |(p, q)| (p, Some(q)));
    (
        path.trim_start_matches('/')
            .split('/')
            .filter(|part| !part.is_empty())
            .collect(),
        query,
    )
}

fn query_param<'a>(query: Option<&'a str>, key: &str) -> Option<&'a str> {
    query?.split('&').find_map(|pair| {
        let (k, v) = pair.split_once('=')?;
        (k == key).then_some(v)
    })
}

fn query_bool(query: Option<&str>, key: &str) -> bool {
    matches!(query_param(query, key), Some("1" | "true" | "yes" | "on"))
}

async fn write_json<T: serde::Serialize>(
    stream: &mut TcpStream,
    value: &T,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let body = serde_json::to_vec(value)?;
    write_response(stream, 200, "application/json; charset=utf-8", &body).await
}

async fn write_download(
    stream: &mut TcpStream,
    filename: &str,
    content_type: &str,
    body: &[u8],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let safe_filename = filename.replace(['"', '\r', '\n'], "_");
    let headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nContent-Disposition: attachment; filename=\"{safe_filename}\"\r\nAccess-Control-Allow-Origin: http://localhost\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(headers.as_bytes()).await?;
    stream.write_all(body).await?;
    Ok(())
}

async fn write_response(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let reason = match status {
        200 => "OK",
        400 => "Bad Request",
        404 => "Not Found",
        _ => "Error",
    };
    let headers = format!(
        "HTTP/1.1 {status} {reason}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: http://localhost\r\nConnection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(headers.as_bytes()).await?;
    stream.write_all(body).await?;
    Ok(())
}

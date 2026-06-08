use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

const MAX_MESSAGE_BYTES: usize = 25 * 1024 * 1024;
const MAX_LINE_BYTES: usize = 1024 * 1024;
const SESSION_TIMEOUT_SECS: u64 = 120;

pub struct SmtpMessage {
    pub from: String,
    pub to: Vec<String>,
    pub data: Vec<u8>,
}

pub struct SmtpSessionResult {
    pub message: Option<SmtpMessage>,
    pub transcript: String,
    pub error: Option<String>,
}

pub async fn handle(stream: TcpStream) -> Result<SmtpSessionResult, Box<dyn std::error::Error + Send + Sync>> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);
    let mut transcript: Vec<String> = Vec::new();

    writer.write_all(b"220 maildrop ESMTP ready\r\n").await?;
    transcript.push("S: 220 maildrop ESMTP ready".to_string());

    let mut from = String::new();
    let mut rcpt_to: Vec<String> = Vec::new();
    let mut data: Vec<u8> = Vec::new();
    let mut in_data = false;
    let mut result: Option<SmtpMessage> = None;
    let mut line = String::new();

    loop {
        line.clear();
        let read = timeout(Duration::from_secs(SESSION_TIMEOUT_SECS), reader.read_line(&mut line)).await;
        let bytes_read = match read {
            Ok(Ok(n)) => n,
            Ok(Err(e)) => return Err(e.into()),
            Err(_) => {
                let error = "session timed out".to_string();
                transcript.push(format!("E: {error}"));
                return Ok(SmtpSessionResult {
                    message: result,
                    transcript: transcript.join("\n"),
                    error: Some(error),
                });
            }
        };
        if bytes_read == 0 {
            break;
        }
        if line.len() > MAX_LINE_BYTES {
            let response = "552 5.3.4 Line too long";
            writer.write_all(format!("{response}\r\n").as_bytes()).await?;
            transcript.push(format!("C: {}", line.trim_end()));
            transcript.push(format!("S: {response}"));
            return Ok(SmtpSessionResult {
                message: result,
                transcript: transcript.join("\n"),
                error: Some("line too long".to_string()),
            });
        }
        if !in_data {
            transcript.push(format!("C: {}", line.trim_end()));
        }

        if in_data {
            let trimmed = line.trim_end_matches(['\r', '\n']);
            if trimmed == "." {
                transcript.push("C: <DATA end>".to_string());
                respond(&mut writer, &mut transcript, "250 2.0.0 OK").await?;
                result = Some(SmtpMessage {
                    from: std::mem::take(&mut from),
                    to: std::mem::take(&mut rcpt_to),
                    data: std::mem::take(&mut data),
                });
                in_data = false;
            } else {
                // Dot-stuffing: leading ".." → "."
                let line_bytes = if line.starts_with("..") {
                    line[1..].as_bytes()
                } else {
                    line.as_bytes()
                };
                data.extend_from_slice(line_bytes);
                if data.len() > MAX_MESSAGE_BYTES {
                    let response = "552 5.3.4 Message size exceeds MailDrop limit";
                    writer.write_all(format!("{response}\r\n").as_bytes()).await?;
                    transcript.push(format!("S: {response}"));
                    return Ok(SmtpSessionResult {
                        message: None,
                        transcript: transcript.join("\n"),
                        error: Some("message size limit exceeded".to_string()),
                    });
                }
            }
        } else {
            let upper = line.trim_end().to_uppercase();

            if upper.starts_with("EHLO") || upper.starts_with("HELO") {
                writer.write_all(b"250-maildrop Hello\r\n250-8BITMIME\r\n250 OK\r\n").await?;
                transcript.push("S: 250-maildrop Hello | 250-8BITMIME | 250 OK".to_string());
            } else if upper.starts_with("MAIL FROM:") {
                from = extract_email(&line);
                respond(&mut writer, &mut transcript, "250 2.1.0 OK").await?;
            } else if upper.starts_with("RCPT TO:") {
                rcpt_to.push(extract_email(&line));
                respond(&mut writer, &mut transcript, "250 2.1.5 OK").await?;
            } else if upper.trim_end() == "DATA" {
                respond(&mut writer, &mut transcript, "354 End data with <CR><LF>.<CR><LF>").await?;
                transcript.push("C: <DATA start>".to_string());
                in_data = true;
            } else if upper.trim_end() == "RSET" {
                from.clear();
                rcpt_to.clear();
                data.clear();
                in_data = false;
                respond(&mut writer, &mut transcript, "250 2.0.0 OK").await?;
            } else if upper.trim_end() == "NOOP" {
                respond(&mut writer, &mut transcript, "250 2.0.0 OK").await?;
            } else if upper.starts_with("QUIT") {
                respond(&mut writer, &mut transcript, "221 2.0.0 Bye").await?;
                break;
            } else if upper.starts_with("STARTTLS") {
                respond(&mut writer, &mut transcript, "454 4.7.0 TLS not available").await?;
            } else if upper.starts_with("AUTH") {
                respond(&mut writer, &mut transcript, "235 2.7.0 Authentication successful").await?;
            } else {
                respond(&mut writer, &mut transcript, "500 5.5.2 Unknown command").await?;
            }
        }
    }

    Ok(SmtpSessionResult {
        message: result,
        transcript: transcript.join("\n"),
        error: None,
    })
}

async fn respond(
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    transcript: &mut Vec<String>,
    line: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    writer.write_all(format!("{line}\r\n").as_bytes()).await?;
    transcript.push(format!("S: {line}"));
    Ok(())
}

fn extract_email(line: &str) -> String {
    if let (Some(s), Some(e)) = (line.find('<'), line.find('>')) {
        if s < e {
            return line[s + 1..e].to_string();
        }
    }
    if let Some(c) = line.find(':') {
        return line[c + 1..].trim().to_string();
    }
    line.trim().to_string()
}

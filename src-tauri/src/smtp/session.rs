use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

pub struct SmtpMessage {
    pub from: String,
    pub to: Vec<String>,
    pub data: Vec<u8>,
}

pub async fn handle(stream: TcpStream) -> Result<Option<SmtpMessage>, Box<dyn std::error::Error + Send + Sync>> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    writer.write_all(b"220 maildrop ESMTP ready\r\n").await?;

    let mut from = String::new();
    let mut rcpt_to: Vec<String> = Vec::new();
    let mut data: Vec<u8> = Vec::new();
    let mut in_data = false;
    let mut result: Option<SmtpMessage> = None;
    let mut line = String::new();

    loop {
        line.clear();
        if reader.read_line(&mut line).await? == 0 {
            break;
        }

        if in_data {
            let trimmed = line.trim_end_matches(['\r', '\n']);
            if trimmed == "." {
                writer.write_all(b"250 2.0.0 OK\r\n").await?;
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
            }
        } else {
            let upper = line.trim_end().to_uppercase();

            if upper.starts_with("EHLO") || upper.starts_with("HELO") {
                writer.write_all(b"250-maildrop Hello\r\n250-8BITMIME\r\n250 OK\r\n").await?;
            } else if upper.starts_with("MAIL FROM:") {
                from = extract_email(&line);
                writer.write_all(b"250 2.1.0 OK\r\n").await?;
            } else if upper.starts_with("RCPT TO:") {
                rcpt_to.push(extract_email(&line));
                writer.write_all(b"250 2.1.5 OK\r\n").await?;
            } else if upper.trim_end() == "DATA" {
                writer.write_all(b"354 End data with <CR><LF>.<CR><LF>\r\n").await?;
                in_data = true;
            } else if upper.trim_end() == "RSET" {
                from.clear();
                rcpt_to.clear();
                data.clear();
                in_data = false;
                writer.write_all(b"250 2.0.0 OK\r\n").await?;
            } else if upper.trim_end() == "NOOP" {
                writer.write_all(b"250 2.0.0 OK\r\n").await?;
            } else if upper.starts_with("QUIT") {
                writer.write_all(b"221 2.0.0 Bye\r\n").await?;
                break;
            } else if upper.starts_with("STARTTLS") {
                writer.write_all(b"454 4.7.0 TLS not available\r\n").await?;
            } else if upper.starts_with("AUTH") {
                writer.write_all(b"235 2.7.0 Authentication successful\r\n").await?;
            } else {
                writer.write_all(b"500 5.5.2 Unknown command\r\n").await?;
            }
        }
    }

    Ok(result)
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

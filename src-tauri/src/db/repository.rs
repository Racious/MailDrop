use crate::db::DbPool;
use crate::models::{
    AppConfig, Mail, MailAttachment, MailAttachmentData, MailSearchResult, MailSummary,
    SmtpSessionLog,
};
use rusqlite::ToSql;

pub type RepoResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

// ── Mail ─────────────────────────────────────────────────────────────────────

pub fn insert_mail(pool: &DbPool, mail: &Mail, attachments: &[MailAttachmentData]) -> RepoResult<()> {
    let mut conn = pool.get()?;
    let to_json = serde_json::to_string(&mail.to_addrs)?;
    let tx = conn.transaction()?;
    tx.execute(
        "INSERT INTO mails
            (id, message_id, from_name, from_addr, to_addrs, subject,
             text_body, html_body, raw_mime, size_bytes, received_at)
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11)",
        rusqlite::params![
            mail.id, mail.message_id, mail.from_name, mail.from_addr,
            to_json, mail.subject, mail.text_body, mail.html_body,
            mail.raw_mime, mail.size_bytes, mail.received_at,
        ],
    )?;
    for attachment in attachments {
        tx.execute(
            "INSERT INTO mail_attachments
                (id, mail_id, filename, content_type, size_bytes, content)
             VALUES (?1,?2,?3,?4,?5,?6)",
            rusqlite::params![
                &attachment.id,
                &mail.id,
                &attachment.filename,
                &attachment.content_type,
                attachment.content.len() as i64,
                &attachment.content,
            ],
        )?;
    }
    tx.commit()?;
    Ok(())
}

pub fn list_mails(pool: &DbPool, offset: u32, limit: u32) -> RepoResult<Vec<MailSummary>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, from_addr, from_name, subject, received_at, size_bytes,
                html_body IS NOT NULL, is_read,
                (SELECT COUNT(*) FROM mail_attachments WHERE mail_id = mails.id)
         FROM mails ORDER BY received_at DESC LIMIT ?1 OFFSET ?2",
    )?;
    let rows = stmt.query_map(rusqlite::params![limit, offset], |row| {
        Ok(MailSummary {
            id: row.get(0)?,
            from_addr: row.get(1)?,
            from_name: row.get(2)?,
            subject: row.get(3)?,
            received_at: row.get(4)?,
            size_bytes: row.get::<_, i64>(5)? as u32,
            has_html: row.get::<_, bool>(6)?,
            is_read: row.get::<_, bool>(7)?,
            attachment_count: row.get::<_, i64>(8)? as u32,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

pub fn search_mails(
    pool: &DbPool,
    query: &str,
    field: &str,
    unread_only: bool,
    has_attachments: bool,
    offset: u32,
    limit: u32,
) -> RepoResult<MailSearchResult> {
    let conn = pool.get()?;
    let mut where_parts = Vec::new();
    let mut values: Vec<Box<dyn ToSql>> = Vec::new();
    let query = query.trim().to_lowercase();

    if !query.is_empty() {
        let like = format!("%{}%", query.replace('%', "\\%").replace('_', "\\_"));
        let fields = match field {
            "from" => vec!["from_addr", "from_name"],
            "to" => vec!["to_addrs"],
            "subject" => vec!["subject"],
            "body" => vec!["text_body", "html_body", "raw_mime"],
            "attachments" => Vec::new(),
            _ => vec![
                "from_addr", "from_name", "to_addrs", "subject", "text_body", "html_body",
                "raw_mime",
            ],
        };

        let mut query_parts = fields
            .iter()
            .map(|column| {
                values.push(Box::new(like.clone()));
                format!("LOWER(COALESCE({column}, '')) LIKE ? ESCAPE '\\'")
            })
            .collect::<Vec<_>>();

        if matches!(field, "all" | "attachments") {
            values.push(Box::new(like.clone()));
            query_parts.push(
                "EXISTS (
                    SELECT 1 FROM mail_attachments a
                    WHERE a.mail_id = mails.id
                      AND LOWER(a.filename || ' ' || a.content_type) LIKE ? ESCAPE '\\'
                )"
                .to_string(),
            );
        }

        where_parts.push(format!("({})", query_parts.join(" OR ")));
    }

    if unread_only {
        where_parts.push("is_read = 0".to_string());
    }

    if has_attachments {
        where_parts.push(
            "EXISTS (SELECT 1 FROM mail_attachments a WHERE a.mail_id = mails.id)".to_string(),
        );
    }

    let where_sql = if where_parts.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", where_parts.join(" AND "))
    };

    let count_sql = format!("SELECT COUNT(*) FROM mails{where_sql}");
    let unread_sql = format!("SELECT COUNT(*) FROM mails{where_sql}{}", if where_parts.is_empty() { " WHERE is_read = 0" } else { " AND is_read = 0" });
    let params = values.iter().map(|v| v.as_ref()).collect::<Vec<&dyn ToSql>>();
    let total_count: i64 = conn.query_row(&count_sql, params.as_slice(), |r| r.get(0))?;
    let unread_count: i64 = conn.query_row(&unread_sql, params.as_slice(), |r| r.get(0))?;
    drop(params);

    let mut item_values = values;
    item_values.push(Box::new(limit.min(500) as i64));
    item_values.push(Box::new(offset as i64));
    let item_params = item_values
        .iter()
        .map(|v| v.as_ref())
        .collect::<Vec<&dyn ToSql>>();
    let sql = format!(
        "SELECT id, from_addr, from_name, subject, received_at, size_bytes,
                html_body IS NOT NULL, is_read,
                (SELECT COUNT(*) FROM mail_attachments WHERE mail_id = mails.id)
         FROM mails{where_sql}
         ORDER BY received_at DESC LIMIT ? OFFSET ?"
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(item_params.as_slice(), mail_summary_from_row)?;
    let items = rows.collect::<Result<Vec<_>, _>>()?;

    Ok(MailSearchResult {
        items,
        total_count: total_count as u32,
        unread_count: unread_count as u32,
    })
}

pub fn mark_as_read(pool: &DbPool, id: &str) -> RepoResult<()> {
    let conn = pool.get()?;
    conn.execute(
        "UPDATE mails SET is_read = 1 WHERE id = ?1",
        rusqlite::params![id],
    )?;
    Ok(())
}

pub fn get_unread_count(pool: &DbPool) -> RepoResult<u32> {
    let conn = pool.get()?;
    let n: i64 = conn.query_row(
        "SELECT COUNT(*) FROM mails WHERE is_read = 0",
        [],
        |r| r.get(0),
    )?;
    Ok(n as u32)
}

pub fn get_mail(pool: &DbPool, id: &str) -> RepoResult<Option<Mail>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, message_id, from_addr, from_name, to_addrs, subject,
                received_at, size_bytes, html_body IS NOT NULL, is_read,
                text_body, html_body, raw_mime,
                (SELECT COUNT(*) FROM mail_attachments WHERE mail_id = mails.id)
         FROM mails WHERE id = ?1",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![id], |row| {
        let to_json: String = row.get(4)?;
        let to_addrs: Vec<String> =
            serde_json::from_str(&to_json).unwrap_or_default();
        Ok(Mail {
            id: row.get(0)?,
            message_id: row.get(1)?,
            from_addr: row.get(2)?,
            from_name: row.get(3)?,
            to_addrs,
            subject: row.get(5)?,
            received_at: row.get(6)?,
            size_bytes: row.get::<_, i64>(7)? as u32,
            has_html: row.get::<_, bool>(8)?,
            is_read: row.get::<_, bool>(9)?,
            text_body: row.get(10)?,
            html_body: row.get(11)?,
            raw_mime: row.get(12)?,
            attachment_count: row.get::<_, i64>(13)? as u32,
            attachments: Vec::new(),
        })
    })?;
    let Some(mut mail) = rows.next().transpose()? else {
        return Ok(None);
    };
    mail.attachments = list_attachments_for_conn(&conn, id)?;
    Ok(Some(mail))
}

pub fn delete_mail(pool: &DbPool, id: &str) -> RepoResult<()> {
    let conn = pool.get()?;
    conn.execute("DELETE FROM mails WHERE id = ?1", rusqlite::params![id])?;
    Ok(())
}

pub fn clear_mails(pool: &DbPool) -> RepoResult<usize> {
    let conn = pool.get()?;
    Ok(conn.execute("DELETE FROM mails", [])?)
}

pub fn list_attachments(pool: &DbPool, mail_id: &str) -> RepoResult<Vec<MailAttachment>> {
    let conn = pool.get()?;
    list_attachments_for_conn(&conn, mail_id)
}

pub fn get_attachment_content(
    pool: &DbPool,
    mail_id: &str,
    attachment_id: &str,
) -> RepoResult<Option<(MailAttachment, Vec<u8>)>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, mail_id, filename, content_type, size_bytes, content
         FROM mail_attachments WHERE mail_id = ?1 AND id = ?2",
    )?;
    let mut rows = stmt.query_map(rusqlite::params![mail_id, attachment_id], |row| {
        Ok((
            MailAttachment {
                id: row.get(0)?,
                mail_id: row.get(1)?,
                filename: row.get(2)?,
                content_type: row.get(3)?,
                size_bytes: row.get::<_, i64>(4)? as u32,
            },
            row.get::<_, Vec<u8>>(5)?,
        ))
    })?;
    Ok(rows.next().transpose()?)
}

pub fn get_mail_count(pool: &DbPool) -> RepoResult<u32> {
    let conn = pool.get()?;
    let n: i64 = conn.query_row("SELECT COUNT(*) FROM mails", [], |r| r.get(0))?;
    Ok(n as u32)
}

pub fn enforce_max_mails(pool: &DbPool, max: usize) -> RepoResult<()> {
    let conn = pool.get()?;
    conn.execute(
        "DELETE FROM mails WHERE id IN (
             SELECT id FROM mails ORDER BY received_at DESC LIMIT -1 OFFSET ?1
         )",
        rusqlite::params![max as i64],
    )?;
    Ok(())
}

pub fn insert_smtp_session(pool: &DbPool, session: &SmtpSessionLog) -> RepoResult<()> {
    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO smtp_sessions
            (id, mail_id, remote_addr, started_at, ended_at, transcript, error)
         VALUES (?1,?2,?3,?4,?5,?6,?7)",
        rusqlite::params![
            &session.id,
            &session.mail_id,
            &session.remote_addr,
            &session.started_at,
            &session.ended_at,
            &session.transcript,
            &session.error,
        ],
    )?;
    Ok(())
}

pub fn list_smtp_sessions(pool: &DbPool, limit: u32) -> RepoResult<Vec<SmtpSessionLog>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, mail_id, remote_addr, started_at, ended_at, transcript, error
         FROM smtp_sessions ORDER BY started_at DESC LIMIT ?1",
    )?;
    let rows = stmt.query_map(rusqlite::params![limit.min(500)], |row| {
        Ok(SmtpSessionLog {
            id: row.get(0)?,
            mail_id: row.get(1)?,
            remote_addr: row.get(2)?,
            started_at: row.get(3)?,
            ended_at: row.get(4)?,
            transcript: row.get(5)?,
            error: row.get(6)?,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

fn list_attachments_for_conn(
    conn: &rusqlite::Connection,
    mail_id: &str,
) -> RepoResult<Vec<MailAttachment>> {
    let mut stmt = conn.prepare(
        "SELECT id, mail_id, filename, content_type, size_bytes
         FROM mail_attachments WHERE mail_id = ?1 ORDER BY filename COLLATE NOCASE",
    )?;
    let rows = stmt.query_map(rusqlite::params![mail_id], |row| {
        Ok(MailAttachment {
            id: row.get(0)?,
            mail_id: row.get(1)?,
            filename: row.get(2)?,
            content_type: row.get(3)?,
            size_bytes: row.get::<_, i64>(4)? as u32,
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
}

fn mail_summary_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<MailSummary> {
    Ok(MailSummary {
        id: row.get(0)?,
        from_addr: row.get(1)?,
        from_name: row.get(2)?,
        subject: row.get(3)?,
        received_at: row.get(4)?,
        size_bytes: row.get::<_, i64>(5)? as u32,
        has_html: row.get::<_, bool>(6)?,
        is_read: row.get::<_, bool>(7)?,
        attachment_count: row.get::<_, i64>(8)? as u32,
    })
}

// ── Config ───────────────────────────────────────────────────────────────────

pub fn get_config(pool: &DbPool) -> RepoResult<AppConfig> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare("SELECT key, value FROM app_config")?;
    let pairs: Vec<(String, String)> = stmt
        .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?
        .collect::<Result<_, _>>()?;

    let mut cfg = AppConfig::default();
    for (k, v) in pairs {
        match k.as_str() {
            "smtp_port"  => cfg.smtp_port  = v.parse().unwrap_or(1025),
            "theme"      => cfg.theme      = v,
            "max_mails"  => cfg.max_mails  = v.parse().unwrap_or(500),
            "check_updates_on_startup" => cfg.check_updates_on_startup = v.parse().unwrap_or(true),
            "auto_install_updates" => cfg.auto_install_updates = v.parse().unwrap_or(false),
            "enable_notifications" => cfg.enable_notifications = v.parse().unwrap_or(true),
            _ => {}
        }
    }
    Ok(cfg)
}

pub fn get_config_value(pool: &DbPool, key: &str) -> RepoResult<String> {
    let conn = pool.get()?;
    let val: String = conn.query_row(
        "SELECT value FROM app_config WHERE key = ?1",
        rusqlite::params![key],
        |r| r.get(0),
    )?;
    Ok(val)
}

pub fn set_config_value(pool: &DbPool, key: &str, value: &str) -> RepoResult<()> {
    let conn = pool.get()?;
    conn.execute(
        "INSERT INTO app_config(key, value) VALUES(?1,?2)
         ON CONFLICT(key) DO UPDATE SET value=excluded.value",
        rusqlite::params![key, value],
    )?;
    Ok(())
}

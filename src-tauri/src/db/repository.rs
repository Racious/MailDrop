use crate::db::DbPool;
use crate::models::{AppConfig, Mail, MailSummary};

pub type RepoResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

// ── Mail ─────────────────────────────────────────────────────────────────────

pub fn insert_mail(pool: &DbPool, mail: &Mail) -> RepoResult<()> {
    let conn = pool.get()?;
    let to_json = serde_json::to_string(&mail.to_addrs)?;
    conn.execute(
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
    Ok(())
}

pub fn list_mails(pool: &DbPool, offset: u32, limit: u32) -> RepoResult<Vec<MailSummary>> {
    let conn = pool.get()?;
    let mut stmt = conn.prepare(
        "SELECT id, from_addr, from_name, subject, received_at, size_bytes,
                html_body IS NOT NULL, is_read
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
        })
    })?;
    Ok(rows.collect::<Result<Vec<_>, _>>()?)
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
                received_at, size_bytes, html_body IS NOT NULL,
                text_body, html_body, raw_mime
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
            text_body: row.get(9)?,
            html_body: row.get(10)?,
            raw_mime: row.get(11)?,
        })
    })?;
    Ok(rows.next().transpose()?)
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

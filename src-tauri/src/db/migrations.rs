use crate::db::DbPool;

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS mails (
    id           TEXT PRIMARY KEY,
    message_id   TEXT,
    from_name    TEXT,
    from_addr    TEXT NOT NULL,
    to_addrs     TEXT NOT NULL,
    subject      TEXT NOT NULL DEFAULT '',
    text_body    TEXT,
    html_body    TEXT,
    raw_mime     TEXT NOT NULL,
    size_bytes   INTEGER NOT NULL DEFAULT 0,
    received_at  TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_mails_received_at ON mails(received_at DESC);

CREATE TABLE IF NOT EXISTS mail_attachments (
    id           TEXT PRIMARY KEY,
    mail_id      TEXT NOT NULL,
    filename     TEXT NOT NULL,
    content_type TEXT NOT NULL,
    size_bytes   INTEGER NOT NULL DEFAULT 0,
    content      BLOB NOT NULL,
    FOREIGN KEY(mail_id) REFERENCES mails(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_mail_attachments_mail_id ON mail_attachments(mail_id);

CREATE TABLE IF NOT EXISTS smtp_sessions (
    id          TEXT PRIMARY KEY,
    mail_id     TEXT,
    remote_addr TEXT NOT NULL,
    started_at  TEXT NOT NULL,
    ended_at    TEXT NOT NULL,
    transcript  TEXT NOT NULL,
    error       TEXT,
    FOREIGN KEY(mail_id) REFERENCES mails(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS idx_smtp_sessions_started_at ON smtp_sessions(started_at DESC);

CREATE TABLE IF NOT EXISTS app_config (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

INSERT OR IGNORE INTO app_config(key, value) VALUES
    ('smtp_port', '1025'),
    ('theme',     'system'),
    ('max_mails', '500'),
    ('check_updates_on_startup', 'true'),
    ('auto_install_updates', 'false');
"#;

pub fn run(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let conn = pool.get()?;
    conn.execute_batch(SCHEMA)?;

    // v0.1.2 — add is_read column (ignore error if already exists)
    let _ = conn.execute_batch(
        "ALTER TABLE mails ADD COLUMN is_read INTEGER NOT NULL DEFAULT 0;"
    );
    let _ = conn.execute_batch(
        "INSERT OR IGNORE INTO app_config(key, value) VALUES ('enable_notifications', 'true');"
    );

    Ok(())
}

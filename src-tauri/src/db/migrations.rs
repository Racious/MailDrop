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

CREATE TABLE IF NOT EXISTS app_config (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);

INSERT OR IGNORE INTO app_config(key, value) VALUES
    ('smtp_port', '1025'),
    ('theme',     'system'),
    ('max_mails', '500');
"#;

pub fn run(pool: &DbPool) -> Result<(), Box<dyn std::error::Error>> {
    let conn = pool.get()?;
    conn.execute_batch(SCHEMA)?;
    Ok(())
}

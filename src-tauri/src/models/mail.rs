use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MailSummary {
    pub id: String,
    pub from_addr: String,
    pub from_name: Option<String>,
    pub subject: String,
    pub received_at: String,
    pub size_bytes: u32,
    pub has_html: bool,
    pub is_read: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mail {
    pub id: String,
    pub message_id: Option<String>,
    pub from_addr: String,
    pub from_name: Option<String>,
    pub subject: String,
    pub received_at: String,
    pub size_bytes: u32,
    pub has_html: bool,
    pub to_addrs: Vec<String>,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
    pub raw_mime: String,
}

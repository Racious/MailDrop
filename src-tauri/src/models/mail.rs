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
    pub attachment_count: u32,
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
    pub is_read: bool,
    pub attachment_count: u32,
    pub to_addrs: Vec<String>,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
    pub raw_mime: String,
    pub attachments: Vec<MailAttachment>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MailAttachment {
    pub id: String,
    pub mail_id: String,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: u32,
}

#[derive(Debug, Clone)]
pub struct MailAttachmentData {
    pub id: String,
    pub filename: String,
    pub content_type: String,
    pub content: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SmtpSessionLog {
    pub id: String,
    pub mail_id: Option<String>,
    pub remote_addr: String,
    pub started_at: String,
    pub ended_at: String,
    pub transcript: String,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MailSearchResult {
    pub items: Vec<MailSummary>,
    pub total_count: u32,
    pub unread_count: u32,
}

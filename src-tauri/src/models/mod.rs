pub mod config;
pub mod mail;

pub use config::AppConfig;
pub use mail::{
    Mail, MailAttachment, MailAttachmentData, MailSearchResult, MailSummary, SmtpSessionLog,
};

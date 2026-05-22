use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub smtp_port: u16,
    pub theme: String,
    pub max_mails: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            smtp_port: 1025,
            theme: "system".to_string(),
            max_mails: 500,
        }
    }
}

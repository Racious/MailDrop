use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppConfig {
    pub smtp_port: u16,
    pub theme: String,
    pub max_mails: u32,
    pub check_updates_on_startup: bool,
    pub auto_install_updates: bool,
    pub enable_notifications: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            smtp_port: 1025,
            theme: "system".to_string(),
            max_mails: 500,
            check_updates_on_startup: true,
            auto_install_updates: false,
            enable_notifications: true,
        }
    }
}

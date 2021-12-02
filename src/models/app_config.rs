use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub pushover: PushoverConfig,
    pub target: String,
    pub timeout_seconds: u64,
    pub relative_store_path: String,
    pub push_message: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PushoverConfig {
    pub user_key: String,
    pub app_token: String,
}
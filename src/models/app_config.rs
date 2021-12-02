use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub pushover: PushoverConfig,
    pub target: String,
    pub timeout_seconds: u64,
    pub relative_store_path: String,
}

#[derive(Debug, Deserialize)]
pub struct PushoverConfig {
    pub user_key: String,
    pub app_token: String,
}
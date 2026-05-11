use serde::Deserialize;

// --- 日志配置 ---
#[derive(Debug, Clone, Deserialize)]
pub struct LoggerConfig {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default = "default_log_path")]
    pub file_path: Option<String>,
}

// --- 默认参数 ---
fn default_log_level() -> String {
    "info".to_string()
}
fn default_log_path() -> Option<String> {
    Some("logs/app.log".to_string())
}

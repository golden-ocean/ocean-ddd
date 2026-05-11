use serde::Deserialize;

// --- 数据库配置 ---
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String, // 必填（缺失时启动失败）
    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout: u64, // 毫秒
}

// --- 默认参数 ---
fn default_min_connections() -> u32 {
    2
}
fn default_max_connections() -> u32 {
    20
}
fn default_connect_timeout() -> u64 {
    10000
} // 10秒

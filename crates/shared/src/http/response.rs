// 获取本机 Hostname (缓存单例，避免每次系统调用)
fn get_hostname() -> &'static str {
    static HOSTNAME: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    HOSTNAME.get_or_init(|| {
        std::env::var("HOSTNAME") // K8s/Docker 默认环境变量
            .or_else(|_| std::env::var("COMPUTERNAME")) // Windows
            .unwrap_or_else(|_| "unknown-host".to_string())
    })
}

// 尝试获取 Trace ID（如集成分布式追踪系统可实现此功能）
fn get_trace_id() -> Option<String> {
    // TODO: 后续集成
    None
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Res<T> {
    pub success: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,

    pub host: &'static str,
    pub timestamp: i64,
}

impl<T> Res<T>
where
    T: serde::Serialize,
{
    // 成功响应
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error_code: None,
            error_message: None,
            trace_id: get_trace_id(),
            host: get_hostname(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }

    // 错误响应
    pub fn err(code: u32, message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error_code: Some(code),
            error_message: Some(message.into()),
            trace_id: get_trace_id(),
            host: get_hostname(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }
}

impl Res<()> {
    // 成功无数据响应
    pub fn success() -> Self {
        Self {
            success: true,
            data: None,
            error_code: None,
            error_message: None,
            trace_id: get_trace_id(),
            host: get_hostname(),
            timestamp: chrono::Utc::now().timestamp_millis(),
        }
    }
}

impl<T: serde::Serialize> axum::response::IntoResponse for Res<T> {
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("参数错误: {0}")]
    BadRequest(String),

    #[error("实体未找到: {0}")]
    NotFound(String),

    #[error("权限不足: {0}")]
    Forbidden(String),

    #[error("认证失败: {0}")]
    Unauthorized(String),

    #[error("业务冲突: {0}")]
    Conflict(String),

    // #[error("数据库错误: {0}")]
    // DatabaseError(#[from] sqlx::Error),

    // #[error("数据库迁移失败: {0}")]
    // MigrationError(#[from] sqlx::migrate::MigrateError),

    // #[error("配置加载失败: {0}")]
    // ConfigError(#[from] config::ConfigError),
    #[error(transparent)]
    InternalError(#[from] anyhow::Error),
}

impl AppError {
    /// 将业务错误映射为 HTTP 状态码
    pub fn status_code(&self) -> axum::http::StatusCode {
        match self {
            Self::BadRequest(_) => axum::http::StatusCode::BAD_REQUEST,
            Self::NotFound(_) => axum::http::StatusCode::NOT_FOUND,
            Self::Forbidden(_) => axum::http::StatusCode::FORBIDDEN,
            Self::Unauthorized(_) => axum::http::StatusCode::UNAUTHORIZED,
            Self::Conflict(_) => axum::http::StatusCode::CONFLICT,
            Self::InternalError(_) => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    /// 获取自定义业务错误码 (用于 Res::error)
    pub fn error_code(&self) -> u32 {
        match self {
            Self::BadRequest(_) => 400,
            Self::NotFound(_) => 404,
            Self::Forbidden(_) => 403,
            Self::Unauthorized(_) => 401,
            Self::Conflict(_) => 409,
            // Self::DatabaseError(_) => 500,
            Self::InternalError(_) => 500,
        }
    }
}

// 实现 axum 的 IntoResponse
impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = self.status_code();
        let code = self.error_code();
        // 针对 InternalError 隐藏具体错误细节
        let message = match self {
            Self::InternalError(ref e) => {
                // 在后端日志记录真实的错误详情（包含 anyhow 的堆栈）
                tracing::error!("系统内部错误: {:?}", e);
                // 只给前端返回模糊的提示
                "服务器内部异常，请联系管理员".to_string()
            }
            // 其他业务逻辑错误，直接返回错误定义中的文本
            _ => self.to_string(),
        };
        let response_body = crate::http::response::Res::<()>::err(code, &message);

        // 返回一个包含 状态码 和 JSON Body 的响应
        (status, response_body).into_response()
    }
}

// /// 统一 Result 定义
// pub type AppResult<T> = Result<T, AppError>;

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use axum::body::to_bytes;
//     use axum::http::StatusCode;
//     use axum::response::IntoResponse;
//     use serde_json::Value;

//     // 1. 基础映射测试 (保留并增强)
//     #[test]
//     fn test_error_status_and_code_mapping() {
//         let err = AppError::NotFound("User not found".to_string());
//         assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
//         assert_eq!(err.error_code(), 404);

//         let anyhow_err = anyhow::anyhow!("DB connection failed");
//         let internal_err = AppError::InternalError(anyhow_err);
//         assert_eq!(
//             internal_err.status_code(),
//             StatusCode::INTERNAL_SERVER_ERROR
//         );
//         assert_eq!(internal_err.error_code(), 500);
//     }

//     // 2. 业务错误响应内容测试
//     // 需要 tokio::test 因为读取 Response Body 是异步操作
//     #[tokio::test]
//     async fn test_business_error_json_response() {
//         let err = AppError::BadRequest("参数格式错误".to_string());
//         let response = err.into_response();

//         assert_eq!(response.status(), StatusCode::BAD_REQUEST);

//         // 将 body 转换为字节并解析为 JSON
//         let body = to_bytes(response.into_body(), 1024).await.unwrap();
//         let json: Value = serde_json::from_slice(&body).unwrap();

//         // 验证 JSON 结构
//         assert_eq!(json["success"], false);
//         assert_eq!(json["errorCode"], 400);
//         // 业务错误应该直接显示：请求错误: 参数格式错误 (由 thiserror 生成)
//         assert!(json["errorMessage"].as_str().unwrap().contains("参数错误"));
//     }

//     // 3. 内部错误隐私屏蔽测试 (最重要)
//     #[tokio::test]
//     async fn test_internal_error_privacy_shield() {
//         let sensitive_msg = "Critical: DB at 192.168.1.100 failed via port 5432";
//         let anyhow_err = anyhow::anyhow!(sensitive_msg);
//         let err = AppError::InternalError(anyhow_err);

//         let response = err.into_response();

//         // --- 重点在这里 ---
//         // 1. 先获取状态码。status() 返回的是 StatusCode
//         // StatusCode 实现了 Copy 特征，所以这里只是简单的值复制，不会移动 response
//         let status = response.status();

//         // 2. 消耗 response 获取 body。这一步之后 response 就不能再用了
//         let body = axum::body::to_bytes(response.into_body(), 1024)
//             .await
//             .unwrap();

//         // 3. 解析 JSON
//         let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

//         // 4. 使用之前保存的 status 进行验证
//         assert_eq!(status, StatusCode::INTERNAL_SERVER_ERROR);

//         // 验证内容
//         let returned_msg = json["errorMessage"].as_str().unwrap();
//         assert!(!returned_msg.contains(sensitive_msg));
//         assert_eq!(returned_msg, "服务器内部异常，请联系管理员");
//     }

//     // 4. 转换测试 (验证 anyhow 能否无缝转为 AppError)
//     #[test]
//     fn test_anyhow_conversion() {
//         fn produce_anyhow_error() -> anyhow::Result<()> {
//             Err(anyhow::anyhow!("anyhow error"))
//         }

//         let result: Result<(), AppError> = produce_anyhow_error().map_err(AppError::from);

//         match result {
//             Err(AppError::InternalError(_)) => (),
//             _ => panic!("应该被转换为 InternalError"),
//         }
//     }
// }

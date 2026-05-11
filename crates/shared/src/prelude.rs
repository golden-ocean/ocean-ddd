// 1. 导出领域与接口类型
pub use crate::http::pagination::{PageQuery, PageRes};
pub use crate::http::response::Res;
pub use crate::types::audit::AuditMetadata;
pub use crate::types::id::Id;
pub use crate::types::status::Status;

// 2. 导出全局错误与结果
pub use crate::error::AppError;

// 3. 导出基础设施
pub use crate::infrastructure::configs::{AppConfig, get_config};
pub use crate::infrastructure::db::init_db;
pub use crate::infrastructure::log::init_logger;

// 4. 导出常用的第三方库工具 (避免在业务 crate 重复引入)
pub use chrono::{DateTime, Utc};
pub use sqlx::{FromRow, PgPool};
pub use tracing::{debug, error, info, warn};
pub use uuid::Uuid;

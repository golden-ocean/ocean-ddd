use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 审计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditMetadata {
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
    // 以后可以扩展字段，例如 version, deleted_at (软删除)
}

impl Default for AuditMetadata {
    fn default() -> Self {
        Self::new(None)
    }
}

impl AuditMetadata {
    pub fn new(creator_id: Option<Uuid>) -> Self {
        let now = Utc::now();
        Self {
            created_at: now,
            updated_at: now,
            created_by: creator_id,
            updated_by: creator_id,
        }
    }
    /// 当实体更新时，调用此方法更新时间戳
    pub fn update(&mut self, operator_id: Option<Uuid>) {
        self.updated_at = Utc::now();
        self.updated_by = operator_id
    }
}

use shared::prelude::{AuditMetadata, DateTime, FromRow, Id, Status, Utc, Uuid};

use crate::dictionary::domain::entity::Dict;
use crate::dictionary::domain::value_object::{DictCode, DictName};

/// 数据字典持久化对象 (Persistent Object)
#[derive(Debug, FromRow)]
pub struct DictModel {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub is_builtin: bool,
    pub sort: i32,
    pub remark: Option<String>,
    pub status: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

// 核心：防腐映射层，将 DB 的脏数据转化为 Domain 纯洁的 Entity
impl From<DictModel> for Dict {
    fn from(po: DictModel) -> Self {
        Self {
            id: Id::from_uuid(po.id),
            // 入库前已经过业务校验，如果查出来格式不对，说明被绕过代码篡改，直接 expect 报警
            code: DictCode::new(po.code).expect("严重错误：DB中字典编码已损坏"),
            name: DictName::new(po.name).expect("严重错误：DB中字典名称已损坏"),
            is_builtin: po.is_builtin,
            sort: po.sort,
            remark: po.remark,
            status: Status::try_from(po.status).expect("严重错误：DB中状态值损坏"),
            audit: AuditMetadata {
                created_at: po.created_at,
                updated_at: po.updated_at,
                created_by: po.created_by,
                updated_by: po.updated_by,
            },
        }
    }
}

impl From<&Dict> for DictModel {
    fn from(entity: &Dict) -> Self {
        Self {
            id: entity.id.value, // 假设你的 Id 包装器内部的 Uuid 字段名为 value
            code: entity.code.as_str().to_string(),
            name: entity.name.as_str().to_string(),
            is_builtin: entity.is_builtin,
            sort: entity.sort,
            remark: entity.remark.clone(),
            status: entity.status as i16,
            created_at: entity.audit.created_at,
            updated_at: entity.audit.updated_at,
            created_by: entity.audit.created_by,
            updated_by: entity.audit.updated_by,
        }
    }
}

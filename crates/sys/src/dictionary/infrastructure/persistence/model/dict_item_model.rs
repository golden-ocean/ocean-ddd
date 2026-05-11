use shared::prelude::{AuditMetadata, DateTime, FromRow, Id, Status, Utc, Uuid};

use crate::dictionary::domain::entity::{Dict, DictItem};
use crate::dictionary::domain::value_object::{DictItemColor, DictItemLabel, DictItemValue};

/// 字典项持久化对象
#[derive(Debug, FromRow)]
pub struct DictItemModel {
    pub id: Uuid,
    pub dict_id: Uuid,
    pub label: String,
    pub value: String,
    pub color: Option<String>,
    pub is_builtin: bool,
    pub sort: i32,
    pub remark: Option<String>,
    pub status: i16,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl From<DictItemModel> for DictItem {
    fn from(po: DictItemModel) -> Self {
        Self {
            id: Id::from_uuid(po.id),
            dict_id: Id::<Dict>::from_uuid(po.dict_id),
            label: DictItemLabel::new(po.label).expect("严重错误：DB中标签已损坏"),
            value: DictItemValue::new(po.value).expect("严重错误：DB中值已损坏"),
            color: po
                .color
                .map(|c| DictItemColor::new(c).expect("严重错误：DB中颜色已损坏")),
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

impl From<&DictItem> for DictItemModel {
    fn from(entity: &DictItem) -> Self {
        Self {
            id: entity.id.value,
            dict_id: entity.dict_id.value,
            label: entity.label.as_str().to_string(),
            value: entity.value.as_str().to_string(),
            color: entity.color.as_ref().map(|c| c.as_str().to_string()),
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

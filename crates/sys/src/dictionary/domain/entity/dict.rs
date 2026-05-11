use serde::{Deserialize, Serialize};
use shared::prelude::{AuditMetadata, Id, Status, Uuid};

use crate::dictionary::domain::entity::AggregateRoot;
use crate::dictionary::domain::error::DictDomianError;
use crate::dictionary::domain::value_object::{DictCode, DictName};

/// 字典聚合根实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dict {
    pub id: Id<Dict>,
    pub code: DictCode,
    pub name: DictName,
    pub is_builtin: bool,
    pub sort: i32,
    pub remark: Option<String>,
    pub status: Status,

    #[serde(flatten)]
    pub audit: AuditMetadata,
}

impl AggregateRoot for Dict {}

impl Dict {
    /// 领域行为：创建一个新的字典
    pub fn new(
        code: DictCode,
        name: DictName,
        sort: i32,
        is_builtin: bool,
        remark: Option<String>,
        creator_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Id::new(),
            code,
            name,
            is_builtin,
            sort,
            remark,
            status: Status::Active,
            audit: AuditMetadata::new(creator_id),
        }
    }

    /// 领域行为：更新字典基本信息
    pub fn update(
        &mut self,
        name: DictName,
        sort: i32,
        remark: Option<String>,
        updater_id: Option<Uuid>,
    ) {
        self.name = name;
        self.sort = sort;
        self.remark = remark;
        self.audit.update(updater_id);
    }

    /// 领域行为：启用字典
    pub fn enable(&mut self, updater_id: Option<Uuid>) {
        self.status = Status::Active;
        self.audit.update(updater_id);
    }

    /// 领域行为：禁用字典
    pub fn disable(&mut self, updater_id: Option<Uuid>) {
        self.status = Status::Inactive;
        self.audit.update(updater_id);
    }

    /// 领域规则校验：是否允许被删除
    pub fn check_can_delete(&self) -> Result<(), DictDomianError> {
        if self.is_builtin {
            return Err(DictDomianError::BuiltInForbidden);
        }
        Ok(())
    }
}

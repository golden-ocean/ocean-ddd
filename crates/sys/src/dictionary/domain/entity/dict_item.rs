use shared::prelude::{AuditMetadata, Id, Status, Uuid};

use crate::dictionary::domain::value_object::{DictItemColor, DictItemLabel, DictItemValue};

/// 数据字典选项
#[derive(Debug, Clone)]
pub struct DictItem {
    pub id: Id<DictItem>,             // 自己的 ID
    pub dict_id: Id<super::Dict>,     // 父聚合的强类型 ID
    pub label: DictItemLabel,         // 展示标签
    pub value: DictItemValue,         // 实际数据值 (不可变)
    pub color: Option<DictItemColor>, // 颜色标识
    pub is_builtin: bool,             // 是否内置
    pub sort: i32,                    // 排序
    pub remark: Option<String>,       // 备注
    pub status: Status,               // 状态

    pub audit: AuditMetadata, // 审计信息
}

impl DictItem {
    /// 领域行为：创建一个新的字典选项
    pub fn new(
        dict_id: Id<super::Dict>,
        label: DictItemLabel,
        value: DictItemValue,
        color: Option<DictItemColor>,
        sort: i32,
        remark: Option<String>,
        creator_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Id::new(),
            dict_id,
            label,
            value,
            color,
            is_builtin: false, // 默认false
            sort,
            remark,
            status: Status::Active, // 默认启用
            audit: AuditMetadata::new(creator_id),
        }
    }

    /// 领域行为：更新字典基本信息
    pub fn update(
        &mut self,
        label: DictItemLabel,
        color: Option<DictItemColor>,
        sort: i32,
        remark: Option<String>,
        updater_id: Option<Uuid>,
    ) {
        self.label = label;
        self.color = color;
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
}

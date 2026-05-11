use async_trait::async_trait;

use shared::prelude::Id;

use crate::dictionary::domain::entity::{Dict, DictItem};
use crate::dictionary::domain::error::DictDomainError;
use crate::dictionary::domain::value_object::{DictCode, DictName};

#[async_trait]
pub trait DictRepository: Send + Sync {
    // 👇 全部统一修改为借用引用：&Id<T>
    async fn get_by_id(&self, id: &Id<Dict>) -> Result<Option<Dict>, DictDomainError>;
    async fn get_by_code(&self, code: &DictCode) -> Result<Option<Dict>, DictDomainError>;
    async fn get_by_name(&self, name: &DictName) -> Result<Option<Dict>, DictDomainError>;
    async fn save(&self, dict: &Dict) -> Result<(), DictDomainError>;
    async fn delete_cascade(&self, dict_id: &Id<Dict>) -> Result<(), DictDomainError>;

    async fn get_item_by_id(&self, id: &Id<DictItem>) -> Result<Option<DictItem>, DictDomainError>;
    async fn get_items_by_dict_id(
        &self,
        dict_id: &Id<Dict>,
    ) -> Result<Vec<DictItem>, DictDomainError>;
    async fn get_items_by_dict_code(
        &self,
        code: &DictCode,
    ) -> Result<Vec<DictItem>, DictDomainError>;
    async fn save_item(&self, item: &DictItem) -> Result<(), DictDomainError>;
    async fn delete_item(&self, id: &Id<DictItem>) -> Result<(), DictDomainError>;
}

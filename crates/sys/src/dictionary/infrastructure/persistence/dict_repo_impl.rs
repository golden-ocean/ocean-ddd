use async_trait::async_trait;
use shared::prelude::{Id, PgPool};

use crate::dictionary::domain::entity::{Dict, DictItem};
use crate::dictionary::domain::error::DictDomainError;
use crate::dictionary::domain::repository::DictRepository;
use crate::dictionary::domain::value_object::{DictCode, DictName};
use crate::dictionary::infrastructure::persistence::model::{DictItemModel, DictModel};

pub struct DictRepositoryImpl {
    pub pool: PgPool,
}

impl DictRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn map_db_error(e: sqlx::Error) -> DictDomainError {
        match &e {
            sqlx::Error::Database(db_err) => {
                // 23505 是 PostgreSQL 的 unique_violation (违反唯一约束)
                if db_err.code().as_deref() == Some("23505") {
                    let msg = db_err.message();
                    if msg.contains("uk_sys_dict_code") {
                        return DictDomainError::CodeAlreadyExists;
                    }
                    if msg.contains("uk_sys_dict_name") {
                        return DictDomainError::NameAlreadyExists;
                    }
                    if msg.contains("uk_sys_dict_item_label") {
                        return DictDomainError::ItemLabelAlreadyExists;
                    }
                    if msg.contains("uk_sys_dict_item_value") {
                        return DictDomainError::ItemValueAlreadyExists;
                    }
                }
                DictDomainError::PersistenceError(db_err.message().to_string())
            }
            sqlx::Error::PoolTimedOut | sqlx::Error::Io(_) => {
                DictDomainError::PersistenceError("数据库连接异常或超时".to_string())
            }
            _ => DictDomainError::PersistenceError(e.to_string()),
        }
    }
}

#[async_trait]
impl DictRepository for DictRepositoryImpl {
    async fn get_by_id(&self, id: &Id<Dict>) -> Result<Option<Dict>, DictDomainError> {
        let po = sqlx::query_as!(
            DictModel,
            r#"SELECT * FROM sys_dict WHERE id = $1"#,
            id.value
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(Self::map_db_error)?;

        Ok(po.map(|p| p.into()))
    }

    async fn get_by_code(&self, code: &DictCode) -> Result<Option<Dict>, DictDomainError> {
        let po = sqlx::query_as!(
            DictModel,
            r#"SELECT * FROM sys_dict WHERE code = $1"#,
            code.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(Self::map_db_error)?;

        Ok(po.map(|p| p.into()))
    }

    async fn get_by_name(&self, name: &DictName) -> Result<Option<Dict>, DictDomainError> {
        let po = sqlx::query_as!(
            DictModel,
            r#"SELECT * FROM sys_dict WHERE name = $1"#,
            name.as_str()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(Self::map_db_error)?;

        Ok(po.map(|p| p.into()))
    }

    async fn save(&self, dict: &Dict) -> Result<(), DictDomainError> {
        let model = DictModel::from(dict);
        // PostgreSQL Upsert (ON CONFLICT DO UPDATE)
        sqlx::query!(
            r#"
            INSERT INTO sys_dict (
                id, code, name, is_builtin, sort, remark, status,
                created_at, updated_at, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ON CONFLICT (id) DO UPDATE SET
                name = EXCLUDED.name,
                sort = EXCLUDED.sort,
                status = EXCLUDED.status,
                remark = EXCLUDED.remark,
                updated_at = EXCLUDED.updated_at,
                updated_by = EXCLUDED.updated_by
            "#,
            model.id,
            model.code,
            model.name,
            model.is_builtin,
            model.sort,
            model.remark,
            model.status,
            model.created_at,
            model.updated_at,
            model.created_by,
            model.updated_by
        )
        .execute(&self.pool)
        .await
        .map_err(Self::map_db_error)?;

        Ok(())
    }

    async fn delete_cascade(&self, dict_id: &Id<Dict>) -> Result<(), DictDomainError> {
        let mut tx = self.pool.begin().await.map_err(Self::map_db_error)?;
        // 1. 先删除子项字典项
        sqlx::query!(
            r#"DELETE FROM sys_dict_item WHERE dict_id = $1"#,
            dict_id.value
        )
        .execute(&mut *tx)
        .await
        .map_err(Self::map_db_error)?;
        // 2. 再删除主字典
        sqlx::query!(r#"DELETE FROM sys_dict WHERE id = $1"#, dict_id.value)
            .execute(&mut *tx)
            .await
            .map_err(Self::map_db_error)?; // 恢复正常的执行、等待和错误映射

        tx.commit().await.map_err(Self::map_db_error)?;

        Ok(())
    }

    async fn get_item_by_id(&self, id: &Id<DictItem>) -> Result<Option<DictItem>, DictDomainError> {
        let po = sqlx::query_as!(
            DictItemModel,
            r#"SELECT * FROM sys_dict_item WHERE id = $1"#,
            id.value
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(Self::map_db_error)?;

        Ok(po.map(|p| p.into()))
    }

    async fn get_items_by_dict_id(
        &self,
        dict_id: &Id<Dict>,
    ) -> Result<Vec<DictItem>, DictDomainError> {
        let po_list = sqlx::query_as!(
            DictItemModel,
            r#"SELECT * FROM sys_dict_item WHERE dict_id = $1 ORDER BY sort ASC, created_at DESC"#,
            dict_id.value
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Self::map_db_error)?;

        Ok(po_list.into_iter().map(|p| p.into()).collect())
    }

    async fn get_items_by_dict_code(
        &self,
        code: &DictCode,
    ) -> Result<Vec<DictItem>, DictDomainError> {
        let po_list = sqlx::query_as!(
            DictItemModel,
            r#"
            SELECT i.* FROM sys_dict_item i
            JOIN sys_dict d ON i.dict_id = d.id
            WHERE d.code = $1
            ORDER BY i.sort ASC, i.created_at DESC
            "#,
            code.as_str()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Self::map_db_error)?;

        Ok(po_list.into_iter().map(|p| p.into()).collect())
    }

    async fn save_item(&self, item: &DictItem) -> Result<(), DictDomainError> {
        let model = DictItemModel::from(item);
        sqlx::query!(
            r#"
            INSERT INTO sys_dict_item (
                id, dict_id, label, value, color, is_builtin, sort, remark, status,
                created_at, updated_at, created_by, updated_by
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (id) DO UPDATE SET
                label = EXCLUDED.label,
                color = EXCLUDED.color,
                sort = EXCLUDED.sort,
                status = EXCLUDED.status,
                remark = EXCLUDED.remark,
                updated_at = EXCLUDED.updated_at,
                updated_by = EXCLUDED.updated_by
            "#,
            model.id,
            model.dict_id,
            model.label,
            model.value,
            model.color,
            model.is_builtin,
            model.sort,
            model.remark,
            model.status,
            model.created_at,
            model.updated_at,
            model.created_by,
            model.updated_by
        )
        .execute(&self.pool)
        .await
        .map_err(Self::map_db_error)?;

        Ok(())
    }

    async fn delete_item(&self, id: &Id<DictItem>) -> Result<(), DictDomainError> {
        sqlx::query!(r#"DELETE FROM sys_dict_item WHERE id = $1"#, id.value)
            .execute(&self.pool)
            .await
            .map_err(Self::map_db_error)?;

        Ok(())
    }
}

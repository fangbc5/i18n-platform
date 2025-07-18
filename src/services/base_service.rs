use async_trait::async_trait;
use serde::Serialize;

use crate::{errors::AppError, repositories::BaseRepository};

#[async_trait]
pub trait BaseService<T>
where
    T: for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Sync + Unpin + Serialize,
{
    type Repository: BaseRepository<T> + Send + Sync;

    fn get_repository(&self) -> &Self::Repository;

    /// 根据ID获取实体
    async fn select_by_id(&self, id: u64) -> Result<Option<T>, AppError> {
        self.get_repository().select_by_id(id).await
    }

    /// 获取所有实体
    async fn select_all(&self) -> Result<Vec<T>, AppError> {
        self.get_repository().select_all().await
    }

    /// 分页获取实体
    async fn select_by_page(&self, page: u32, page_size: u32) -> Result<(Vec<T>, i64), AppError> {
        self.get_repository().select_by_page(page, page_size).await
    }

    /// 根据列名和值获取实体列表
    async fn select_by_column(&self, column: &str, value: &str) -> Result<Vec<T>, AppError> {
        self.get_repository().select_by_column(column, value).await
    }

    /// 根据列名和值获取单个实体
    async fn select_by_column_optional(
        &self,
        column: &str,
        value: &str,
    ) -> Result<Option<T>, AppError> {
        self.get_repository()
            .select_by_column_optional(column, value)
            .await
    }

    /// 创建实体
    async fn insert(&self, entity: &T) -> Result<u64, AppError> {
        self.get_repository().insert(entity).await
    }

    /// 更新实体
    async fn update_by_id(&self, id: u64, entity: &T) -> Result<bool, AppError> {
        self.get_repository().update_by_id(id, entity).await
    }

    /// 删除实体
    async fn delete_by_id(&self, id: u64) -> Result<bool, AppError> {
        self.get_repository().delete_by_id(id).await
    }

    /// 批量删除实体
    async fn delete_by_ids(&self, ids: &[u64]) -> Result<u64, AppError> {
        self.get_repository().delete_by_ids(ids).await
    }
}

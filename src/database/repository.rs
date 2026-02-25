use async_trait::async_trait;
use std::sync::Arc;

pub type ArcRepository<T, ID> = Arc<dyn Repository<T, ID> + Send + Sync>;

#[async_trait]
pub trait ReadRepository<T, ID> {
    async fn find_by_id(&self, id: &ID) -> Option<T>;

    async fn find_all(&self) -> Vec<T>;
}

#[async_trait]
pub trait WriteRepository<T, ID> {
    async fn create(&self, entity: &T) -> Option<T>;

    async fn update(&self, entity: &T) -> Option<T>;

    async fn delete_by_id(&self, id: &ID) -> u64;
}

#[cfg(test)]
#[async_trait]
pub trait TruncateRepository {
    async fn truncate(&self) -> u64;
}

pub trait Repository<T, ID>: ReadRepository<T, ID> + WriteRepository<T, ID> {}

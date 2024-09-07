use std::sync::Arc;

use axum::async_trait;
use mockall::automock;

use crate::entities::items::Items;

pub type SharedItemsRepository = Arc<dyn ItemsRepository + Send + Sync>;

#[async_trait]
#[automock]
pub trait ItemsRepository {
    async fn find_by_name(&self, name: String) -> Result<Items, sqlx::Error>;
    async fn insert(&self, item: Items) -> Result<i32, sqlx::Error>;
    async fn find_by_id(&self, id: i32) -> Result<Items, sqlx::Error>;
}

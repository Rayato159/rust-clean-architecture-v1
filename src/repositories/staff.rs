use super::items::{ItemsRepository, SharedItemsRepository};
use crate::entities::items::Items;
use axum::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::error;

pub struct StaffRepository {
    db_pool: PgPool,
}

impl StaffRepository {
    pub fn creation(db_pool: PgPool) -> SharedItemsRepository {
        Arc::new(Self { db_pool })
    }
}

#[async_trait]
impl ItemsRepository for StaffRepository {
    async fn find_by_name(&self, name: String) -> Result<Items, sqlx::Error> {
        let item = match sqlx::query_as::<_, Items>(
            "SELECT * FROM items WHERE (name = $1 AND category = 'Staff');",
        )
        .bind(name.clone())
        .fetch_one(&self.db_pool)
        .await
        {
            Ok(item) => item,
            Err(e) => {
                error!("Failed to find item by name: {}", e);
                return Err(e);
            }
        };

        Ok(item)
    }

    async fn insert(&self, item: Items) -> Result<i32, sqlx::Error> {
        let item = match sqlx::query_as::<_, Items>(
            "INSERT INTO items (name, category, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *;",
        )
        .bind(item.name)
        .bind(item.category)
        .bind(item.created_at)
        .bind(item.updated_at)
        .fetch_one(&self.db_pool)
        .await
        {
            Ok(item) => item,
            Err(e) => {
                error!("Failed to insert item: {:?}", e);
                return Err(e);
            }
        };

        Ok(match item.id {
            Some(id) => id,
            None => {
                error!("Failed to insert item: id is missing");
                return Err(sqlx::Error::RowNotFound);
            }
        })
    }

    async fn find_by_id(&self, id: i32) -> Result<Items, sqlx::Error> {
        let item = match sqlx::query_as::<_, Items>(
            "SELECT * FROM items WHERE (id = $1 AND category = 'Staff');",
        )
        .bind(id)
        .fetch_one(&self.db_pool)
        .await
        {
            Ok(item) => item,
            Err(e) => {
                error!("Failed to find item by id: {}", e);
                return Err(e);
            }
        };

        Ok(item)
    }
}

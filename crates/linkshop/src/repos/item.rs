use sqlx::PgPool;
use uuid::Uuid;

use crate::{domain::Item, error::AppError, repos::models::ItemRow};

#[allow(async_fn_in_trait)]
pub trait ItemRepository: Send + Sync {
    async fn list_by_business_id(&self, business_id: Uuid) -> Result<Vec<Item>, AppError>;
    async fn insert(&self, item: &Item) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct PgItemRepository {
    pool: PgPool,
}

impl PgItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl ItemRepository for PgItemRepository {
    async fn list_by_business_id(&self, business_id: Uuid) -> Result<Vec<Item>, AppError> {
        let items = sqlx::query_as::<_, ItemRow>(
            r#"
            SELECT id, business_id, name, description, price, is_available, sort_order, created_at, updated_at
            FROM items
            WHERE business_id = $1
            ORDER BY sort_order ASC, created_at ASC
            "#,
        )
        .bind(business_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(items.into_iter().map(Item::from).collect())
    }

    async fn insert(&self, item: &Item) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO items (id, business_id, name, description, price, is_available, sort_order, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
        )
        .bind(item.id)
        .bind(item.business_id)
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.price)
        .bind(item.is_available)
        .bind(item.sort_order)
        .bind(item.created_at)
        .bind(item.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

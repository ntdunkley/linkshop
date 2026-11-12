use sqlx::PgPool;

use crate::{domain::Business, error::AppError};

#[allow(async_fn_in_trait)]
pub trait BusinessRepository: Send + Sync {
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Business>, AppError>;
    async fn find_published_by_slug(&self, slug: &str) -> Result<Option<Business>, AppError>;
    async fn insert(&self, business: &Business) -> Result<(), AppError>;
}

#[derive(Clone)]
pub struct PgBusinessRepository {
    pool: PgPool,
}

impl PgBusinessRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl BusinessRepository for PgBusinessRepository {
    async fn find_by_slug(&self, slug: &str) -> Result<Option<Business>, AppError> {
        let business = sqlx::query_as::<_, Business>(
            r#"
            SELECT id, name, slug, description, is_published, created_at, updated_at
            FROM businesses
            WHERE slug = $1
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(business)
    }

    async fn find_published_by_slug(&self, slug: &str) -> Result<Option<Business>, AppError> {
        let business = sqlx::query_as::<_, Business>(
            r#"
            SELECT id, name, slug, description, is_published, created_at, updated_at
            FROM businesses
            WHERE slug = $1 AND is_published = TRUE
            "#,
        )
        .bind(slug)
        .fetch_optional(&self.pool)
        .await?;

        Ok(business)
    }

    async fn insert(&self, business: &Business) -> Result<(), AppError> {
        sqlx::query(
            r#"
            INSERT INTO businesses (id, name, slug, description, is_published, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(business.id)
        .bind(&business.name)
        .bind(&business.slug)
        .bind(&business.description)
        .bind(business.is_published)
        .bind(business.created_at)
        .bind(business.updated_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

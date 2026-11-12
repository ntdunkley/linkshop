use std::env;
use std::time::Duration;

use chrono::Utc;
use sqlx::{PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

use crate::{
    domain::{Business, Item},
    repos::{BusinessRepository, ItemRepository, PgBusinessRepository, PgItemRepository},
};

pub struct TestContext {
    pub pool: PgPool,
    pub business_repo: PgBusinessRepository,
    pub item_repo: PgItemRepository,
}

impl TestContext {
    pub async fn try_new() -> Option<Self> {
        let _ = dotenv::dotenv();

        let database_url = env::var("TEST_DATABASE_URL")
            .or_else(|_| env::var("DATABASE_URL"))
            .unwrap_or_else(|_| {
                "postgres://postgres:password@localhost:5432/linkshop_test".to_string()
            });

        let pool = match PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(2))
            .connect(&database_url)
            .await
        {
            Ok(p) => p,
            Err(err) => {
                eprintln!(
                    "PostgreSQL not reachable at {} ({}). Test database skipped.",
                    database_url, err
                );
                return None;
            }
        };

        // Run migrations on the test database
        if let Err(err) = sqlx::migrate!("./migrations").run(&pool).await {
            eprintln!("Failed to apply migrations on test database: {}", err);
            return None;
        }

        let ctx = Self {
            pool: pool.clone(),
            business_repo: PgBusinessRepository::new(pool.clone()),
            item_repo: PgItemRepository::new(pool),
        };

        // Clean tables to ensure an isolated test environment
        ctx.clean_tables().await;

        Some(ctx)
    }

    pub async fn clean_tables(&self) {
        let _ = sqlx::query("TRUNCATE TABLE items, businesses CASCADE;")
            .execute(&self.pool)
            .await;
    }

    pub async fn create_business(
        &self,
        name: &str,
        slug: &str,
        description: Option<&str>,
        is_published: bool,
    ) -> Business {
        let now = Utc::now();
        let business = Business {
            id: Uuid::new_v4(),
            name: name.to_string(),
            slug: slug.to_string(),
            description: description.map(|s| s.to_string()),
            is_published,
            created_at: now,
            updated_at: now,
        };

        self.business_repo
            .insert(&business)
            .await
            .expect("Failed to insert test business");

        business
    }

    pub async fn create_item(
        &self,
        business_id: Uuid,
        name: &str,
        description: Option<&str>,
        price: i64,
        is_available: bool,
        sort_order: i32,
    ) -> Item {
        let now = Utc::now();
        let item = Item {
            id: Uuid::new_v4(),
            business_id,
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            price,
            is_available,
            sort_order,
            created_at: now,
            updated_at: now,
        };

        self.item_repo
            .insert(&item)
            .await
            .expect("Failed to insert test item");

        item
    }
}

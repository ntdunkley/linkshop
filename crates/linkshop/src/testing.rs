use std::{env, net::TcpListener, time::Duration};

use actix_web::{
    App,
    body::BoxBody,
    dev::{ServiceFactory, ServiceRequest, ServiceResponse},
};
use chrono::Utc;
use sqlx::{Connection, PgConnection, PgPool, postgres::PgPoolOptions};
use uuid::Uuid;

use crate::{
    domain::{Business, Item},
    repos::{BusinessRepository, ItemRepository, PgBusinessRepository, PgItemRepository},
    startup::{build_app, run, run_migrations},
};

/// Resolves the test database URL from the environment.
fn test_database_url() -> String {
    let _ = dotenv::dotenv();

    env::var("TEST_DATABASE_URL")
        .or_else(|_| env::var("DATABASE_URL"))
        .unwrap_or_else(|_| "postgres://postgres:password@localhost:5432/linkshop_test".to_string())
}

/// An isolated PostgreSQL schema that lives for the duration of a single test.
///
/// Each test gets its own schema so tests can run in parallel without truncating
/// (and therefore corrupting) data belonging to other tests. The schema is
/// dropped when the guard is dropped.
struct TestSchema {
    database_url: String,
    name: String,
}

impl TestSchema {
    async fn create(database_url: &str) -> Self {
        let name = format!("test_{}", Uuid::new_v4().simple());

        let mut connection = PgConnection::connect(database_url).await.unwrap_or_else(|err| {
            panic!(
                "Failed to connect to test database at {}: {}. Ensure PostgreSQL is running (e.g. `docker compose up -d postgres`).",
                database_url, err
            );
        });

        sqlx::query(sqlx::AssertSqlSafe(format!("CREATE SCHEMA \"{name}\"")))
            .execute(&mut connection)
            .await
            .expect("Failed to create isolated test schema");

        Self {
            database_url: database_url.to_string(),
            name,
        }
    }

    /// Builds a pool whose connections resolve tables within this schema.
    async fn pool(&self) -> PgPool {
        let schema = self.name.clone();

        PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .after_connect(move |connection, _metadata| {
                let schema = schema.clone();
                Box::pin(async move {
                    sqlx::query(sqlx::AssertSqlSafe(format!(
                        "SET search_path TO \"{schema}\""
                    )))
                    .execute(connection)
                    .await?;
                    Ok(())
                })
            })
            .connect(&self.database_url)
            .await
            .unwrap_or_else(|err| {
                panic!(
                    "Failed to connect to test database at {}: {}",
                    self.database_url, err
                );
            })
    }
}

impl Drop for TestSchema {
    fn drop(&mut self) {
        let database_url = self.database_url.clone();
        let schema = self.name.clone();

        // Cleanup runs on its own runtime because this guard is dropped inside
        // the test's async runtime, which may already be shutting down.
        let cleanup = std::thread::spawn(move || {
            let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            else {
                return;
            };

            runtime.block_on(async move {
                let Ok(mut connection) = PgConnection::connect(&database_url).await else {
                    return;
                };

                let _ = sqlx::query(sqlx::AssertSqlSafe(format!(
                    "DROP SCHEMA IF EXISTS \"{schema}\" CASCADE"
                )))
                .execute(&mut connection)
                .await;
            });
        });

        let _ = cleanup.join();
    }
}

/// Builds a pool with its own schema and applies migrations to it.
async fn isolated_pool(database_url: &str) -> (PgPool, TestSchema) {
    let schema = TestSchema::create(database_url).await;
    let pool = schema.pool().await;

    if let Err(err) = run_migrations(&pool).await {
        panic!(
            "Failed to apply migrations to isolated test schema {} at {}: {}",
            schema.name, database_url, err
        );
    }

    (pool, schema)
}

/// TestContext encapsulates an isolated database connection and repositories
/// for integration testing against a real PostgreSQL instance.
pub struct TestContext {
    pub pool: PgPool,
    pub business_repo: PgBusinessRepository,
    pub item_repo: PgItemRepository,
    // Held to drop the isolated schema once the test finishes.
    _schema: TestSchema,
}

impl TestContext {
    /// Creates a new TestContext backed by a schema unique to this test.
    /// Panics immediately if the database is unreachable or migrations fail.
    pub async fn new() -> Self {
        let database_url = test_database_url();
        let (pool, schema) = isolated_pool(&database_url).await;

        Self {
            business_repo: PgBusinessRepository::new(pool.clone()),
            item_repo: PgItemRepository::new(pool.clone()),
            pool,
            _schema: schema,
        }
    }

    /// Builds the Actix application service configured exactly as in production.
    pub fn app(
        &self,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Config = (),
            Response = ServiceResponse<BoxBody>,
            Error = actix_web::Error,
            InitError = (),
        > + use<>,
    > {
        build_app(self.pool.clone())
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

/// TestApp represents a fully running HTTP server bound to an ephemeral port,
/// executing the identical server pipeline used in production.
pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub pool: PgPool,
    pub business_repo: PgBusinessRepository,
    pub item_repo: PgItemRepository,
    pub server_task: tokio::task::JoinHandle<Result<(), std::io::Error>>,
    // Held to drop the isolated schema once the test finishes.
    _schema: TestSchema,
}

impl TestApp {
    /// Spawns the application server on an ephemeral port, backed by a schema
    /// unique to this test.
    /// Panics immediately if the database is unreachable or the server cannot start.
    pub async fn spawn() -> Self {
        let database_url = test_database_url();
        let (pool, schema) = isolated_pool(&database_url).await;

        let listener = TcpListener::bind("127.0.0.1:0")
            .expect("Failed to bind ephemeral TCP port for TestApp");

        let port = listener
            .local_addr()
            .expect("Failed to read local address for TestApp")
            .port();

        let server = run(listener, pool.clone()).expect("Failed to start HTTP server for TestApp");

        let server_task = tokio::spawn(server);

        Self {
            address: format!("http://127.0.0.1:{}", port),
            port,
            business_repo: PgBusinessRepository::new(pool.clone()),
            item_repo: PgItemRepository::new(pool.clone()),
            pool,
            server_task,
            _schema: schema,
        }
    }

    pub async fn clean_tables(&self) {
        let _ = sqlx::query("TRUNCATE TABLE items, businesses CASCADE;")
            .execute(&self.pool)
            .await;
    }
}

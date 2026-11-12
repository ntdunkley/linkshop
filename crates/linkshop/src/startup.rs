use std::{net::TcpListener, time::Duration};

use actix_web::{
    App, HttpServer,
    body::BoxBody,
    dev::{Server, ServiceFactory, ServiceRequest, ServiceResponse},
    web,
};
use sqlx::{PgPool, postgres::PgPoolOptions};

use crate::{
    config::AppConfig,
    handlers,
    repos::{PgBusinessRepository, PgItemRepository},
};

pub struct Application {
    port: u16,
    server: Server,
    pool: PgPool,
}

impl Application {
    pub async fn build(config: AppConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let pool = get_connection_pool(&config.database_url).await?;
        run_migrations(&pool).await?;

        let address = format!("{}:{}", config.host, config.port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr()?.port();
        let server = run(listener, pool.clone())?;

        Ok(Self { port, server, pool })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || build_app(pool.clone()))
        .listen(listener)?
        .run();
    Ok(server)
}

pub fn build_app(
    pool: PgPool,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<BoxBody>,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    let business_repo = PgBusinessRepository::new(pool.clone());
    let item_repo = PgItemRepository::new(pool.clone());

    App::new()
        .app_data(web::Data::new(pool))
        .app_data(web::Data::new(business_repo))
        .app_data(web::Data::new(item_repo))
        .configure(handlers::register_routes)
}

pub async fn get_connection_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(3))
        .connect(database_url)
        .await
}

pub async fn run_migrations(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

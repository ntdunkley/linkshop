use actix_web::HttpServer;
use linkshop::{build_app, config::AppConfig, telemetry::init_telemetry};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_telemetry();

    let config = match AppConfig::from_env() {
        Ok(cfg) => cfg,
        Err(err) => {
            tracing::error!(error = %err, "Failed to load application configuration");
            return Err(Box::new(err) as Box<dyn std::error::Error>);
        }
    };

    tracing::info!("Connecting to database at configured DATABASE_URL");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
    {
        Ok(p) => p,
        Err(err) => {
            tracing::error!(error = %err, "Failed to connect to database");
            return Err(Box::new(err) as Box<dyn std::error::Error>);
        }
    };

    tracing::info!("Running database migrations");
    if let Err(err) = sqlx::migrate!("./migrations").run(&pool).await {
        tracing::error!(error = %err, "Failed to apply database migrations");
        return Err(Box::new(err) as Box<dyn std::error::Error>);
    }

    let bind_addr = (config.host.as_str(), config.port);
    tracing::info!(host = %config.host, port = config.port, "Starting HTTP server");

    HttpServer::new(move || build_app(pool.clone()))
        .bind(bind_addr)?
        .run()
        .await?;

    Ok(())
}

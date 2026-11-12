use linkshop::{Application, config::AppConfig, telemetry::init_telemetry};

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

    let app = match Application::build(config).await {
        Ok(a) => a,
        Err(err) => {
            tracing::error!(error = %err, "Failed to initialize application");
            return Err(err);
        }
    };

    tracing::info!(port = app.port(), "Starting HTTP server");
    if let Err(err) = app.run_until_stopped().await {
        tracing::error!(error = %err, "Application server encountered an unrecoverable error");
        return Err(Box::new(err) as Box<dyn std::error::Error>);
    }

    Ok(())
}

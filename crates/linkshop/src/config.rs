use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Missing required environment variable: {0}")]
    MissingVar(String),

    #[error("Invalid port value: {0}")]
    InvalidPort(String),
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let _ = dotenv::dotenv();

        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());

        let port = match env::var("PORT") {
            Ok(val) => val
                .parse::<u16>()
                .map_err(|_| ConfigError::InvalidPort(val))?,
            Err(_) => 8080,
        };

        let database_url = env::var("DATABASE_URL")
            .map_err(|_| ConfigError::MissingVar("DATABASE_URL".to_string()))?;

        Ok(Self {
            host,
            port,
            database_url,
        })
    }
}

use config::{Config, ConfigError, Environment};
use serde::Deserialize;
use std::env;

use super::database::DatabaseConfig;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub api_base_path: String,
    pub service_name: String,
    pub environment: String,
}

impl AppConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config = Config::builder()
            // Set defaults
            .set_default("server.port", 8080)?
            .set_default("server.host", "0.0.0.0")?
            .set_default("server.api_base_path", "/v1")?
            .set_default("server.service_name", "rust-api")?
            .set_default("server.environment", "local")?
            // Load from environment variables
            .add_source(Environment::default().separator("_"))
            .build()?;

        let mut app_config: AppConfig = config.try_deserialize()?;

        // Handle DATABASE_URL specifically
        if let Ok(database_url) = env::var("DATABASE_URL") {
            app_config.database.url = database_url;
        }

        Ok(app_config)
    }
}


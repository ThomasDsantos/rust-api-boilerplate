use sea_orm::{Database, DatabaseConnection, DbErr};
use tracing::{info, error};
use crate::config::DatabaseConfig;

pub async fn connect(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    info!("Connecting to database: {}", &config.url);
    
    let db = Database::connect(&config.url).await?;
    
    // Test the connection
    match db.ping().await {
        Ok(_) => {
            info!("Successfully connected to database");
            Ok(db)
        }
        Err(e) => {
            error!("Failed to ping database: {}", e);
            Err(e)
        }
    }
}

pub async fn health_check(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.ping().await?;
    Ok(())
}


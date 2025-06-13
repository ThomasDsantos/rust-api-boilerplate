use migration::{Migrator, MigratorTrait};
use sea_orm::DatabaseConnection;
use tracing::{info, error};

pub async fn run_migrations(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    info!("Starting database migrations...");
    
    match Migrator::up(db, None).await {
        Ok(_) => {
            info!("Database migrations completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Failed to run database migrations: {}", e);
            Err(e)
        }
    }
}


use sea_orm::{DatabaseConnection, DbErr};

pub async fn health_check(db: &DatabaseConnection) -> Result<(), DbErr> {
    db.ping().await?;
    Ok(())
}


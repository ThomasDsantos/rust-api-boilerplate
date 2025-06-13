use crate::{
    database,
    models::HealthResponse,
    error::AppError,
};
use axum::{
    extract::State,
    Json,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn health(
    State(db): State<Arc<DatabaseConnection>>,
) -> Result<Json<HealthResponse>, AppError> {
    // Test database connection
    database::health_check(&db).await?;

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        service: "rust-api-boilerplate".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    }))
}


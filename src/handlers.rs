use crate::{
    database,
    entities::{visit, Visit},
    error::AppError,
    models::{GreetingOutput, HealthResponse},
};
use axum::{
    extract::{ConnectInfo, Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

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

/// Greeting endpoint
#[utoipa::path(
    get,
    path = "/greet/{name}",
    params(
        ("name" = String, Path, description = "Name to greet")
    ),
    responses(
        (status = 200, description = "Greeting response", body = GreetingOutput)
    )
)]
pub async fn greet(
    Path(name): Path<String>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(db): State<Arc<DatabaseConnection>>,
) -> Result<Json<GreetingOutput>, AppError> {
    info!("test");
    info!("Greeting request from {} for name: {}", addr.ip(), name);

    // Record the visit
    let visit = visit::ActiveModel {
        ip: Set(addr.ip().to_string()),
        name: Set(name.clone()),
        visited_at: Set(Utc::now()),
    };

    // Insert visit record
    let _inserted = visit.insert(&*db).await?;

    let message = format!("Hello, {}!", name);

    Ok(Json(GreetingOutput { message }))
}

/// Get all visits
#[utoipa::path(
    get,
    path = "/visits",
    responses(
        (status = 200, description = "List of visits", body = [visit::VisitResponse])
    )
)]
pub async fn get_visits(
    State(db): State<Arc<DatabaseConnection>>,
) -> Result<Json<Vec<visit::VisitResponse>>, AppError> {
    let visits = Visit::find().all(&*db).await?;

    let response: Vec<visit::VisitResponse> =
        visits.into_iter().map(visit::VisitResponse::from).collect();

    Ok(Json(response))
}

/// Create a new visit
#[utoipa::path(
    post,
    path = "/visits",
    request_body = visit::CreateVisitRequest,
    responses(
        (status = 201, description = "Visit created", body = visit::VisitResponse)
    )
)]
pub async fn create_visit(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<visit::CreateVisitRequest>,
) -> Result<(StatusCode, Json<visit::VisitResponse>), AppError> {
    let visit = visit::ActiveModel {
        ip: Set(addr.ip().to_string()),
        name: Set(payload.name),
        visited_at: Set(Utc::now()),
    };

    let inserted = visit.insert(&*db).await?;
    let response = visit::VisitResponse::from(inserted);

    Ok((StatusCode::CREATED, Json(response)))
}

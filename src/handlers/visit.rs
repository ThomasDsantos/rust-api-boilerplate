use crate::{
    models::{VisitResponse, CreateVisitRequest},
    services::VisitService,
    error::AppError,
};
use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    Json,
};
use sea_orm::DatabaseConnection;
use std::{net::SocketAddr, sync::Arc};

/// Get all visits
#[utoipa::path(
    get,
    path = "/visits",
    responses(
        (status = 200, description = "List of visits", body = [VisitResponse])
    )
)]
pub async fn get_visits(
    State(db): State<Arc<DatabaseConnection>>,
) -> Result<Json<Vec<VisitResponse>>, AppError> {
    let visit_service = VisitService::new(db);
    let visits = visit_service.get_all_visits().await?;
    Ok(Json(visits))
}

/// Create a new visit
#[utoipa::path(
    post,
    path = "/visits",
    request_body = CreateVisitRequest,
    responses(
        (status = 201, description = "Visit created", body = VisitResponse)
    )
)]
pub async fn create_visit(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(db): State<Arc<DatabaseConnection>>,
    Json(payload): Json<CreateVisitRequest>,
) -> Result<(StatusCode, Json<VisitResponse>), AppError> {
    let visit_service = VisitService::new(db);
    let visit = visit_service.create_visit(&payload.name, addr.ip()).await?;
    Ok((StatusCode::CREATED, Json(visit)))
}


use crate::{error::AppError, models::GreetingOutput, services::GreetingService};
use axum::{
    extract::{ConnectInfo, Path, State},
    Json,
};
use sea_orm::DatabaseConnection;
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

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
    info!("Greeting request from {} for name: {}", addr.ip(), name);

    let greeting_service = GreetingService::new(db);
    let response = greeting_service
        .greet_and_record_visit(&name, addr.ip())
        .await?;

    Ok(Json(response))
}

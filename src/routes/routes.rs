use crate::{
    config::AppConfig,
    entities::visit,
    handlers,
    models::{ErrorResponse, GreetingOutput, HealthResponse},
};
use axum::{
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::health,
        handlers::greet,
        handlers::get_visits,
        handlers::create_visit,
    ),
    components(
        schemas(
            HealthResponse,
            GreetingOutput,
            ErrorResponse,
            visit::VisitResponse,
            visit::CreateVisitRequest,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "greetings", description = "Greeting endpoints"),
        (name = "visits", description = "Visit management endpoints"),
    )
)]
pub struct ApiDoc;

pub fn create_routes(db: Arc<DatabaseConnection>, config: &AppConfig) -> Router {
    let api_routes = Router::new()
        .route("/health", get(handlers::health))
        .route("/greet/:name", get(handlers::greet))
        .route("/visits", get(handlers::get_visits))
        .route("/visits", post(handlers::create_visit))
        .with_state(db);

    // Create the main router with API prefix
    let app = Router::new()
        .nest(&config.server.api_base_path, api_routes)
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()));

    app
}


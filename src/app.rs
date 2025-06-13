use crate::{
    config::AppConfig,
    database,
    error::AppError,
    routes::create_routes,
    middleware::setup_middleware,
};
use axum::Router;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::info;

pub struct App {
    pub config: AppConfig,
    pub db: Arc<DatabaseConnection>,
}

impl App {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        // Connect to database
        let db = database::connect(&config.database).await?;
        let db = Arc::new(db);

        Ok(Self { config, db })
    }

    pub async fn start(&self) -> Result<(), AppError> {
        let router = self.create_router().await;
        
        let addr = format!("{}:{}", self.config.server.host, self.config.server.port);
        info!("Starting server on {}", addr);
        
        let listener = TcpListener::bind(&addr)
            .await
            .map_err(|e| AppError::internal_server_error(format!("Failed to bind to {}: {}", addr, e)))?;

        axum::serve(listener, router.into_make_service_with_connect_info::<std::net::SocketAddr>())
            .await
            .map_err(|e| AppError::internal_server_error(format!("Server error: {}", e)))?;

        Ok(())
    }

    async fn create_router(&self) -> Router {
        let router = create_routes(self.db.clone(), &self.config);
        setup_middleware(router)
    }
}


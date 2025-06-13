use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use sea_orm::DbErr;
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] DbErr),
    
    #[error("Configuration error: {0}")]
    Config(#[from] config::ConfigError),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Bad request: {0}")]
    BadRequest(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
}

impl AppError {
    pub fn internal_server_error(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }
    
    pub fn bad_request(msg: impl Into<String>) -> Self {
        Self::BadRequest(msg.into())
    }
    
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::Config(err) => {
                tracing::error!("Configuration error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::Internal(msg) => {
                tracing::error!("Internal error: {}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
            AppError::BadRequest(msg) => {
                tracing::warn!("Bad request: {}", msg);
                (StatusCode::BAD_REQUEST, msg)
            }
            AppError::NotFound(msg) => {
                tracing::warn!("Not found: {}", msg);
                (StatusCode::NOT_FOUND, msg)
            }
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}


use tokio::signal;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::io::{stdout, Write};

mod app;
mod config;
mod database;
mod handlers;
mod models;
mod routes;
mod middleware;
mod error;
mod entities;

use app::App;
use config::AppConfig;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenvy::dotenv().ok();

    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_api_boilerplate=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = match AppConfig::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    // Create and start the application
    let app = match App::new(config).await {
        Ok(app) => app,
        Err(e) => {
            error!("Failed to create application: {}", e);
            std::process::exit(1);
        }
    };

    // Start the server
    if let Err(e) = app.start().await {
        error!("Failed to start application: {}", e);
        std::process::exit(1);
    }

    // Wait for shutdown signal
    shutdown_signal().await;
    info!("Shutdown signal received, stopping server...");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}


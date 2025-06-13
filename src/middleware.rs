use axum::{
    http::Request,
    middleware::{self, Next},
    response::Response,
    Router,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::info;

pub fn setup_middleware(router: Router) -> Router {
    router
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any))
                .layer(middleware::from_fn(request_logging))
        )
}

async fn request_logging(req: Request<axum::body::Body>, next: Next) -> Response {
    let method = req.method().clone();
    let uri = req.uri().clone();
    
    info!("Incoming request: {} {}", method, uri);
    
    let response = next.run(req).await;
    
    info!("Response status: {}", response.status());
    
    response
}


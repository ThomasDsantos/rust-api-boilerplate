use axum::{
    extract::{ConnectInfo, FromRequestParts, rejection::ExtensionRejection},
    http::request::Parts,
};
use std::net::{IpAddr, SocketAddr};

/// Custom extractor for client IP address
pub struct ClientIp(pub IpAddr);

#[axum::async_trait]
impl<S> FromRequestParts<S> for ClientIp
where
    S: Send + Sync,
{
    type Rejection = ExtensionRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ConnectInfo(addr) = ConnectInfo::<SocketAddr>::from_request_parts(parts, state).await?;
        Ok(ClientIp(addr.ip()))
    }
}


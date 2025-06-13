use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::entities::visit;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct VisitResponse {
    pub ip: String,
    pub name: String,
    pub visited_at: DateTime<Utc>,
}

impl From<visit::Model> for VisitResponse {
    fn from(visit: visit::Model) -> Self {
        Self {
            ip: visit.ip,
            name: visit.name,
            visited_at: visit.visited_at,
        }
    }
}

#[derive(Deserialize, ToSchema)]
pub struct CreateVisitRequest {
    pub name: String,
}


use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "visit")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub ip: String,
    
    #[sea_orm(primary_key, auto_increment = false)]
    pub visited_at: DateTime<Utc>,
    
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

// DTOs for API
#[derive(Serialize, Deserialize, ToSchema)]
pub struct VisitResponse {
    pub ip: String,
    pub name: String,
    pub visited_at: DateTime<Utc>,
}

impl From<Model> for VisitResponse {
    fn from(visit: Model) -> Self {
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


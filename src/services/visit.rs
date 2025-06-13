use crate::{
    entities::{visit, Visit},
    models::VisitResponse,
    error::AppError,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use std::{net::IpAddr, sync::Arc};

pub struct VisitService {
    db: Arc<DatabaseConnection>,
}

impl VisitService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn get_all_visits(&self) -> Result<Vec<VisitResponse>, AppError> {
        let visits = Visit::find().all(&*self.db).await?;
        let response: Vec<VisitResponse> = 
            visits.into_iter().map(VisitResponse::from).collect();
        Ok(response)
    }

    pub async fn create_visit(&self, name: &str, ip: IpAddr) -> Result<VisitResponse, AppError> {
        let visit = visit::ActiveModel {
            ip: Set(ip.to_string()),
            name: Set(name.to_string()),
            visited_at: Set(Utc::now()),
        };

        let inserted = visit.insert(&*self.db).await?;
        let response = VisitResponse::from(inserted);
        Ok(response)
    }
}


use crate::{
    entities::visit,
    models::GreetingOutput,
    error::AppError,
};
use chrono::Utc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use std::{net::IpAddr, sync::Arc};

pub struct GreetingService {
    db: Arc<DatabaseConnection>,
}

impl GreetingService {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        Self { db }
    }

    pub async fn greet_and_record_visit(&self, name: &str, ip: IpAddr) -> Result<GreetingOutput, AppError> {
        // Record the visit
        let visit = visit::ActiveModel {
            ip: Set(ip.to_string()),
            name: Set(name.to_string()),
            visited_at: Set(Utc::now()),
        };

        // Insert visit record
        let _inserted = visit.insert(&*self.db).await?;

        let message = format!("Hello, {}!", name);

        Ok(GreetingOutput { message })
    }
}


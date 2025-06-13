use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://admin:password@localhost:5432/database?sslmode=disable".to_string(),
        }
    }
}


use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct GreetingOutput {
    pub message: String,
}

#[derive(Deserialize, ToSchema)]
pub struct GreetingInput {
    pub name: String,
}


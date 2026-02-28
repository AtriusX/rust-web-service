use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

mod claims;

pub use claims::*;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginDto {
    pub user_name: String,
    pub password: String,
}
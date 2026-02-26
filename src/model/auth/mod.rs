use serde::{Deserialize, Serialize};

mod claims;

pub use claims::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginDto {
    pub user_name: String,
    pub password: String,
}
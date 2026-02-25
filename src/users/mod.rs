mod user;
mod user_controller;
mod user_manager;
mod user_repository;

use crate::users::user_manager::UserManager;
use crate::users::user_repository::UserRepository;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct UsersApi {
    user_manager: UserManager,
}

impl UsersApi {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            user_manager: UserManager::new(Arc::new(UserRepository::new(pool))),
        }
    }
}

pub use user_controller::get_routes;

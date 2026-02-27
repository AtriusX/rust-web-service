use crate::manager::UserManager;
use crate::repository::UserRepository;
use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct UsersApi {
    pub user_repository: Arc<UserRepository>,
    pub user_manager: UserManager,
}

impl UsersApi {
    pub fn new(pool: &PgPool) -> Self {
        let user_repository = Arc::new(UserRepository::new(pool));
        let user_manager = UserManager::new(user_repository.clone());

        Self {
            user_repository,
            user_manager
        }
    }
}

use crate::manager::UserManager;
use crate::repository::UserRepository;
use crate::state::AppState;
use axum::extract::FromRef;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct UsersApi {
    pub user_manager: UserManager,
}

impl UsersApi {
    pub fn new(pool: &PgPool) -> Self {
        let user_repository = Arc::new(UserRepository::new(pool));
        let user_manager = UserManager::new(user_repository.clone());

        Self {
            user_manager
        }
    }
}

impl FromRef<AppState> for UserManager {
    fn from_ref(input: &AppState) -> Self {
        input.users_api.user_manager.clone()
    }
}
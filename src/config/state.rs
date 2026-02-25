use crate::users::UsersApi;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub users_api: UsersApi,
}

impl AppState {
    pub fn new(pool: &PgPool) -> Self {
        Self {
            users_api: UsersApi::new(pool),
        }
    }
}

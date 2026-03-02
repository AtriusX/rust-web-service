mod users_api;

use crate::manager::AuthManager;
use crate::services::refresh_token_service::RefreshTokenService;
use crate::services::AuthService;
pub use crate::state::users_api::UsersApi;
use axum::extract::FromRef;
use deadpool_redis::Pool;
use log::info;
use sqlx::{migrate, PgPool};
use std::io::Error;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub users_api: UsersApi,
    pub auth_manager: AuthManager,
    pub auth_service: AuthService,
}

impl AppState {
    pub async fn new(pg_pool: &PgPool, redis_pool: Pool) -> Self {
        Self::configure(pg_pool, redis_pool).await
    }

    async fn configure(pg_pool: &PgPool, redis_pool: Pool) -> Self {
        // Run database migrations
        info!("Running database migrations...");
        let _ = migrate!("./migrations")
            .run(pg_pool)
            .await
            .map_err(Error::other);
        info!("Done!");

        let users_api = UsersApi::new(pg_pool);
        let auth_service = AuthService::new(
            RefreshTokenService::new(redis_pool),
        );
        let auth_manager = AuthManager::new(auth_service.clone());

        Self {
            users_api,
            auth_manager,
            auth_service,
        }
    }
}

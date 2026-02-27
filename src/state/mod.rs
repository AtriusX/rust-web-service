mod users_api;

use crate::services::AuthService;
pub(crate) use crate::state::users_api::UsersApi;
use axum::extract::FromRef;
use log::info;
use sqlx::postgres::PgPoolOptions;
use sqlx::{migrate, PgPool};
use std::env;
use std::io::{Error, ErrorKind};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub users_api: UsersApi,
    pub auth_service: AuthService,
}

impl AppState {
    pub async fn new(pool: PgPool) -> Self {
        info!("Running database migrations...");
        let _ = migrate!("./migrations")
            .run(&pool)
            .await
            .map_err(Error::other);
        info!("Done!");

        let users_api = UsersApi::new(&pool);
        let auth_service = AuthService::new();

        Self {
            users_api,
            auth_service
        }
    }

    pub async fn get_pool() -> Result<PgPool, Error> {
        let db_url = env::var("DATABASE_URL")
            .map_err(|_| Error::new(ErrorKind::NotConnected, "DATABASE_URL not set"))?;

        PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .map_err(Error::other)
    }
}
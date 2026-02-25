mod state;

use axum::Router;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
pub use state::AppState;
use std::env;
use std::io::{Error, ErrorKind};
use log::debug;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn get_tracing() {
    let format = tracing_subscriber::fmt::layer();
    let filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("rust_playground=error,tower_http=warn"))
        .unwrap();

    tracing_subscriber::registry()
        .with(format)
        .with(filter.clone())
        .init();

    debug!("Initializing logger with settings: {}", filter);
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

pub fn app(pool: &PgPool, routers: Vec<Router<AppState>>) -> Router {
    routers
        .into_iter()
        .fold(Router::new(), Router::merge)
        .with_state(AppState::new(pool))
        .layer(TraceLayer::new_for_http())
}

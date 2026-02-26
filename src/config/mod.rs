pub mod authentication;

use crate::state::AppState;
use axum::Router;
use log::debug;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

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

pub async fn app(pool: PgPool, routers: Vec<Router<AppState>>) -> Router {
    let state = AppState::new(pool).await;
    let router = routers
        .into_iter()
        .fold(Router::new(), Router::merge)
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    router
}

pub mod authentication;

use crate::middleware::auth_layer;
use crate::state::AppState;
use axum::http::Method;
use axum::{middleware, Router};
use log::debug;
use sqlx::PgPool;
use tower_http::cors::{AllowMethods, AllowOrigin, CorsLayer};
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

pub async fn app(
    pool: PgPool,
    protected_routers: Vec<Router<AppState>>,
    public_routers: Vec<Router<AppState>>,
) -> Router {
    let state = AppState::new(pool).await;
    let protected = protected_routers
        .into_iter()
        .fold(Router::new(), Router::merge)
        .layer(middleware::from_fn(auth_layer));
    let public = public_routers
        .into_iter()
        .fold(Router::new(), Router::merge);
    let cors = CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap()))
        .allow_methods(AllowMethods::list(
            vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        );

    Router::new()
        .merge(protected)
        .merge(public)
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

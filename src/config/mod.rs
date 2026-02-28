pub mod authentication;
pub mod openapi;

use crate::config::openapi::OpenApiSpec;
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
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

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

fn get_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::exact("http://localhost:3000".parse().unwrap()))
        .allow_methods(AllowMethods::list(
            vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        )
}

fn get_swagger(
    protected_api: utoipa::openapi::OpenApi,
    public_api: utoipa::openapi::OpenApi,
) -> SwaggerUi {
    let api = OpenApiSpec::openapi()
        .merge_from(protected_api)
        .merge_from(public_api);
    let swagger = SwaggerUi::new("/swagger-ui")
        .url("/api.json", api)
        .config(utoipa_swagger_ui::Config::default().persist_authorization(true));

    swagger
}

pub async fn app(
    pool: PgPool,
    protected_routers: Vec<OpenApiRouter<AppState>>,
    public_routers: Vec<OpenApiRouter<AppState>>,
) -> Router {
    let (protected_router, protected_api) = protected_routers
        .into_iter()
        .fold(OpenApiRouter::new(), OpenApiRouter::merge)
        .layer(middleware::from_fn(auth_layer))
        .split_for_parts();
    let (public_router, public_api) = public_routers
        .into_iter()
        .fold(OpenApiRouter::new(), OpenApiRouter::merge)
        .split_for_parts();
    let swagger = get_swagger(protected_api, public_api);
    let state = AppState::new(pool).await;
    let cors = get_cors();

    Router::new()
        .merge(protected_router)
        .merge(public_router)
        .merge(swagger)
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
}

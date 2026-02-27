mod config;
mod util;
mod services;
mod model;
mod state;
mod manager;
mod repository;
mod controller;
mod middleware;

use crate::state::AppState;
use log::info;
use tokio::net::TcpListener;

#[tokio::main]
#[warn(clippy::nursery)]
#[deny(clippy::all, clippy::pedantic)]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    config::get_tracing();

    let routes = vec![
        controller::user_controller::get_routes(),
        controller::auth_controller::get_protected_routes(),
    ];
    let public_routes = vec![
        controller::auth_controller::get_routes(),
    ];
    let pool = AppState::get_pool().await?;
    let app = config::app(pool, routes, public_routes).await;
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    let addr = listener.local_addr()?;

    info!("Serving app on {addr}");

    axum::serve(listener, app.into_make_service()).await
}

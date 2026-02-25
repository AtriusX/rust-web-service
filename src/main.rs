mod config;
pub mod database;
mod users;
mod util;

use log::info;
use sqlx::migrate;
use std::io::Error;
use tokio::net::TcpListener;

#[tokio::main]
#[warn(clippy::nursery)]
#[deny(clippy::all, clippy::pedantic)]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    config::get_tracing();

    let pool = config::get_pool().await?;

    info!("Running database migrations...");
    migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(Error::other)?;
    info!("Done!");

    let routes = vec![users::get_routes()];
    let app = config::app(&pool, routes);
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    let addr = listener.local_addr()?;

    info!("Serving app on {addr}");

    axum::serve(listener, app.into_make_service()).await
}

use deadpool::Runtime;
use deadpool_redis::{Config, Pool};
use log::debug;
use redis::AsyncCommands;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::io::{Error, ErrorKind};

pub struct StorageConfig;

impl StorageConfig {
    pub async fn get_pg_pool() -> Result<PgPool, Error> {
        let db_url = env::var("DATABASE_URL")
            .map_err(|_| Error::new(ErrorKind::NotConnected, "DATABASE_URL not set"))?;

        PgPoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await
            .map_err(Error::other)
    }

    pub async fn get_redis_pool() -> Result<Pool, Error> {
        let redis_url = env::var("REDIS_URL")
            .map_err(|_| Error::new(ErrorKind::NotConnected, "REDIS_URL not set"))?;
        let config = Config::from_url(&redis_url);
        let pool = config.create_pool(Some(Runtime::Tokio1))
            .map_err(|_| Error::new(ErrorKind::NotConnected, "Failed to create pool"))?;

        debug!("Attempting connection to Redis...");
        {
            let mut conn = pool.get().await
                .expect("Redis failed to get connection from pool");
            let ping = conn
                .ping_message::<&str, String>("ok").await
                .expect("Redis service failed to response");

            assert_eq!(ping, "ok");
        }
        debug!("Succeeded in connecting to Redis service!");

        Ok(pool)
    }
}
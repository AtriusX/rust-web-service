use crate::config::environment;
use deadpool_redis::{Connection, Pool, PoolError};
use log::debug;
use redis::AsyncCommands;
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Clone)]
pub struct RefreshTokenService {
    redis_pool: Pool,
}

impl RefreshTokenService {
    pub fn new(redis_pool: Pool) -> Self {
        Self { redis_pool }
    }

    pub async fn generate_refresh_token(&self, user_id: &str) -> Result<Uuid, PoolError> {
        let token = Uuid::new_v4();

        debug!("Generating refresh token for user {user_id}");
        {
            let key = self.get_key(&token.to_string());
            let mut conn = self.get_connection().await?;
            let _ = conn
                .set_ex::<String, &str, ()>(key, user_id, *environment::REFRESH_TOKEN_EXP_DAYS).await;
        }

        Ok(token)
    }

    pub async fn validate_refresh_token(&self, token: &Uuid) -> Result<String, PoolError> {
        let key = self.get_key(&token.to_string());
        let mut conn = self.get_connection().await?;
        let user_id = conn
            .get::<_, String>(&key).await?;

        debug!("Found refresh token for user {user_id} validated successfully");
        // Immediately invalidate the refresh token upon receiving the user_id back
        self.invalidate_token(&mut conn, &key).await?;
        Ok(user_id)
    }

    pub async fn invalidate_refresh_token(&self, token: &str) -> Result<(), PoolError> {
        let mut conn = self.get_connection().await?;
        let key = self.get_key(token);

        self.invalidate_token(&mut conn, &key).await
    }

    async fn invalidate_token(&self, conn: &mut Connection, key: &String) -> Result<(), PoolError> {
        debug!("Invalidating refresh token to prevent reuse");
        let _ = conn.del::<_, ()>(key).await;
        Ok(())
    }

    async fn get_connection(&self) -> Result<Connection, PoolError> {
        self.redis_pool.get().await
    }

    fn get_key(&self, key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        format!("refresh_token:{:x}", hasher.finalize())
    }
}
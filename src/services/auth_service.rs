use crate::config::authentication::KEYS;
use crate::config::environment;
use crate::model::auth::JwtClaims;
use crate::model::auth_error::AuthError;
use crate::services::refresh_token_service::RefreshTokenService;
use crate::util;
use jsonwebtoken::{encode, Header};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AuthBody {
    pub access_token: String,
    pub refresh_token: Uuid,
    pub token_type: String,
}

impl AuthBody {
    fn new(access_token: String, refresh_token: Uuid) -> Self {
        Self {
            access_token,
            refresh_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct AuthService {
    refresh_token_service: RefreshTokenService,
}

impl AuthService {
    pub fn new(refresh_token_service: RefreshTokenService) -> Self {
        Self {
            refresh_token_service,
        }
    }

    pub async fn generate_login_tokens(&self, user_id: &str) -> Result<AuthBody, AuthError> {
        let claims = JwtClaims {
            sub: user_id.to_owned(),
            exp: util::now_epoch() + *environment::ACCESS_TOKEN_EXP_MINUTES,
        };
        let access_token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| AuthError::TokenCreation)?;
        let refresh_token = self
            .refresh_token_service
            .generate_refresh_token(user_id)
            .await
            .map_err(|_| AuthError::TokenCreation)?;

        Ok(AuthBody::new(access_token, refresh_token))
    }

    pub async fn refresh_login(&self, refresh_token: Uuid) -> Result<AuthBody, AuthError> {
        let user_id = self
            .refresh_token_service
            .validate_refresh_token(&refresh_token)
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // The token came back as valid, so we can issue a fresh token pair
        self.generate_login_tokens(&user_id).await
    }

    pub async fn invalidate_login(&self, refresh_token: &str) -> Result<(), AuthError> {
        let _ = self.refresh_token_service.invalidate_refresh_token(refresh_token).await;
        Ok(())
    }
}

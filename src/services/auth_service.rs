use crate::config::authentication::KEYS;
use crate::model::auth::JwtClaims;
use crate::model::auth_error::AuthError;
use crate::util;
use jsonwebtoken::{encode, Header};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Default)]
pub struct AuthService;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}
impl AuthBody {
    fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}


impl AuthService {
    const ACCESS_EXP_MINUTES: u32 = 15 * 60;

    pub fn new() -> Self {
        Self {}
    }

    pub fn generate_tokens(&self, user_id: &str) -> Result<AuthBody, AuthError> {
        let claims = JwtClaims {
            sub: user_id.to_owned(),
            exp: util::now_epoch() + AuthService::ACCESS_EXP_MINUTES as usize,
        };

        let access_token = encode(&Header::default(), &claims, &KEYS.encoding)
            .map_err(|_| AuthError::TokenCreation)?;

        Ok(AuthBody::new(access_token))
    }
}

use crate::config::authentication::KEYS;
use crate::model::auth::JwtClaims;
use crate::model::auth_error::AuthError;
use crate::util;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, encode, Header, Validation};
use serde::Serialize;

#[derive(Clone, Default)]
pub struct AuthService;

#[derive(Debug, Serialize)]
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

impl<S> FromRequestParts<S> for JwtClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        let token_data = decode::<JwtClaims>(bearer.token(), &KEYS.decoding, &Validation::default())
            .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}
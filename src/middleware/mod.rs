use axum::Extension;
use crate::config::authentication::KEYS;
use crate::model::auth::JwtClaims;
use crate::model::auth_error::AuthError;
use axum::extract::{FromRef, Request};
use axum::middleware::Next;
use axum::response::Response;
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::{Authorization, HeaderMapExt};
use jsonwebtoken::{decode, Validation};

pub async fn auth_layer(
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let auth = request.headers()
        .typed_get::<Authorization<Bearer>>()
        .ok_or(AuthError::MissingCredentials)?;
    let claims = decode::<JwtClaims>(auth.token(), &KEYS.decoding, &Validation::default())
        .map_err(|_| AuthError::InvalidToken)?;

    request.extensions_mut().insert(claims.claims.clone());
    Ok(next.run(request).await)
}

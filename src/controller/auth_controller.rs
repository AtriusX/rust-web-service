use crate::model::auth::{JwtClaims, LoginDto};
use crate::model::auth_error::AuthError;
use crate::services::{AuthBody, AuthService};
use crate::state::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
}

pub fn get_protected_routes() -> Router<AppState> {
    Router::new()
        .route("/get-user-info", get(get_info))
}

async fn login(
    State(state): State<AuthService>,
    Json(payload): Json<LoginDto>,
) -> (StatusCode, Json<Result<AuthBody, AuthError>>) {
    if payload.user_name.is_empty() || payload.password.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(Err(AuthError::MissingCredentials)));
    }

    if payload.user_name != "foo" || payload.password != "bar" {
        return (StatusCode::BAD_REQUEST, Json(Err(AuthError::WrongCredentials)));
    }

    let tokens = state.generate_tokens(payload.user_name.as_str());

    match tokens {
        Ok(v) => (StatusCode::OK, Json(Ok(v))),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, Json(Err(AuthError::TokenCreation))),
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserInfo {
    pub username: String,
    pub email: String,
    pub info: String,
}

async fn get_info(
    Extension(claims): Extension<JwtClaims>,
) -> (StatusCode, Json<Result<GetUserInfo, AuthError>>) {
    (StatusCode::OK, Json(Ok(
        GetUserInfo {
            username: claims.sub,
            email: "foo@foo.com".to_string(),
            info: "Hello there!".to_string(),
        }
    )))
}
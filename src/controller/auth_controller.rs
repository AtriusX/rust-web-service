use crate::model::api_response::{ApiResponse, AsApiResponse};
use crate::model::auth::{JwtClaims, LoginDto};
use crate::model::auth_error::AuthError;
use crate::services::{AuthBody, AuthService};
use crate::state::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

const AUTH_TAG: &str = "Authorization";

pub fn get_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(login))
}

pub fn get_protected_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(get_info))
}

#[utoipa::path(
    post,
    path = "/login",
    responses(
        (status = 200, body = AuthBody),
        (status = 400, body = AuthError),
        (status = 500, body = AuthError),
    ),
    tag = AUTH_TAG,
    security(),
)]
async fn login(
    State(state): State<AuthService>,
    Json(payload): Json<LoginDto>,
) -> ApiResponse<AuthBody, AuthError> {
    if payload.user_name.is_empty() || payload.password.is_empty() {
        return Err(AuthError::MissingCredentials).as_api_response_ok();
    }

    if payload.user_name != "foo" || payload.password != "bar" {
        return Err(AuthError::WrongCredentials).as_api_response_ok();
    }

    state
        .generate_tokens(payload.user_name.as_str())
        .as_api_response_ok()
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct GetUserInfo {
    pub username: String,
    pub email: String,
    pub info: String,
}

#[utoipa::path(
    post,
    path = "/get-user-info",
    responses(
        (status = 200, body = GetUserInfo),
        (status = 401, body = AuthError),
    ),
    tag = AUTH_TAG,
)]
async fn get_info(
    Extension(claims): Extension<JwtClaims>,
) -> ApiResponse<GetUserInfo, AuthError> {
    Ok(
        GetUserInfo {
            username: claims.sub,
            email: "foo@foo.com".to_string(),
            info: "Hello there!".to_string(),
        }
    ).as_api_response_ok()
}
use crate::model::api_response::{ApiError, AsApiError, ResponseError};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Error)]
pub enum AuthError {
    #[error("Wrong credentials provided!")]
    WrongCredentials,
    #[error("Credentials not provided!")]
    MissingCredentials,
    #[error("Error occurred in login token creation.")]
    TokenCreation,
    #[error("Invalid session token, please log back in.")]
    InvalidToken,
    #[error("Failed to generate refresh token: {0}")]
    RefreshTokenCreation(String),
}

impl ResponseError for AuthError {

    fn to_api_err_response(&self) -> (StatusCode, ApiError) {
        match self {
            Self::WrongCredentials =>
                self.as_api_error(StatusCode::BAD_REQUEST, "WrongCredentials"),
            Self::MissingCredentials =>
                self.as_api_error(StatusCode::BAD_REQUEST, "MissingCredentials"),
            Self::InvalidToken =>
                self.as_api_error(StatusCode::UNAUTHORIZED, "InvalidToken"),
            Self::TokenCreation =>
                self.as_api_error(StatusCode::INTERNAL_SERVER_ERROR, "TokenCreation"),
            Self::RefreshTokenCreation(_) =>
                self.as_api_error(StatusCode::INTERNAL_SERVER_ERROR, "RefreshTokenCreation"),
        }
    }
}

impl IntoResponse for AuthError {

    fn into_response(self) -> Response {
        let (code, err) = self.to_api_err_response();
        (code, Json(err)).into_response()
    }
}
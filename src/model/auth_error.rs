use crate::model::api_response::{ApiError, AsApiError, ResponseError};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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
}

impl ResponseError for AuthError {

    fn to_api_err_response(&self) -> (StatusCode, ApiError) {
        match self {
            AuthError::WrongCredentials =>
                self.as_api_error(StatusCode::BAD_REQUEST, "WrongCredentials"),
            AuthError::MissingCredentials =>
                self.as_api_error(StatusCode::BAD_REQUEST, "MissingCredentials"),
            AuthError::InvalidToken =>
                self.as_api_error(StatusCode::UNAUTHORIZED, "InvalidToken"),
            AuthError::TokenCreation =>
                self.as_api_error(StatusCode::UNAUTHORIZED, "InvalidToken"),
        }
    }
}

impl IntoResponse for AuthError {

    fn into_response(self) -> Response {
        self.to_api_err_response().into_response()
    }
}
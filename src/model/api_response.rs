use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub type ApiResponse<T> = (StatusCode, Result<Json<T>, Json<ApiError>>);

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiError {
    pub code: String,
    pub message: String,
}

pub trait AsApiError {

    fn as_api_error(&self, status_code: StatusCode, code: &str) -> (StatusCode, ApiError);
}

impl<T : ToString> AsApiError for T {

    fn as_api_error(&self, status_code: StatusCode, code: &str) -> (StatusCode, ApiError) {
        let error = ApiError {
            code: code.to_string(),
            message: self.to_string(),
        };

        (status_code, error)
    }
}

pub trait ResponseError {
    fn to_api_err_response(&self) -> (StatusCode, ApiError);
}

pub trait AsApiResponse<T> {
    fn as_api_response_ok(&self) -> ApiResponse<T> {
        self.as_api_response(StatusCode::OK)
    }

    fn as_api_response(&self, ok_status: StatusCode) -> ApiResponse<T>;
}

impl<T, E> AsApiResponse<T> for Result<T, E>
where
    T: Clone,
    E: Clone + ResponseError,
{
    fn as_api_response(&self, ok_status: StatusCode) -> ApiResponse<T> {
        self.clone()
            .map(|t| (ok_status, Ok(Json(t))))
            .unwrap_or_else(|e| {
                let (status, err) = e.to_api_err_response();
                (status, Err(Json(err)))
            })
    }
}

impl IntoResponse for ApiError {

    fn into_response(self) -> Response {
        todo!()
    }
}
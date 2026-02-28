use axum::http::StatusCode;
use axum::Json;

pub type ApiResponse<T, E> = (StatusCode, Result<Json<T>, Json<E>>);

pub trait ResponseError {
    fn to_status_code(&self) -> StatusCode;
}

pub trait AsApiResponse<T, E> {
    fn as_api_response_ok(&self) -> ApiResponse<T, E> {
        self.as_api_response(StatusCode::OK)
    }

    fn as_api_response(&self, ok_status: StatusCode) -> ApiResponse<T, E>;
}

impl<T, E> AsApiResponse<T, E> for Result<T, E>
where
    T: Clone,
    E: Clone + ResponseError,
{
    fn as_api_response(&self, ok_status: StatusCode) -> ApiResponse<T, E> {
        self.clone()
            .map(|t| (ok_status, Ok(Json(t))))
            .unwrap_or_else(|e| (e.to_status_code(), Err(Json(e))))
    }
}

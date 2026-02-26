use axum::http::StatusCode;
use axum::Json;
use std::time::{SystemTime, UNIX_EPOCH};

pub trait AsDtoEnabled<T> {
    fn as_dto(&self) -> T;

    fn from_dto(dto: &T) -> Self;
}

pub trait ToJson<T: Clone, E: Clone> {
    fn to_json_ok(&self, err: StatusCode) -> (StatusCode, Json<Result<T, E>>) {
        self.to_json(StatusCode::OK, err)
    }

    fn to_json(&self, ok: StatusCode, err: StatusCode) -> (StatusCode, Json<Result<T, E>>);
}

impl<T: Clone, E: Clone> ToJson<T, E> for Result<T, E> {
    fn to_json(&self, ok: StatusCode, err: StatusCode) -> (StatusCode, Json<Self>) {
        match self {
            Ok(_) => (ok, Json(self.clone())),
            Err(_) => (err, Json(self.clone())),
        }
    }
}

pub fn now_epoch() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}


use std::time::{SystemTime, UNIX_EPOCH};

pub trait AsDtoEnabled<T> {
    fn as_dto(&self) -> T;

    fn from_dto(dto: &T) -> Self;
}

pub fn now_epoch() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}


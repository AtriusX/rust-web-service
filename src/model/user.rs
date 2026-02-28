use crate::util::AsDtoEnabled;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Clone, FromRow)]
#[allow(dead_code)]
#[allow(clippy::struct_field_names)]
pub struct User {
    pub id: Option<i32>,
    pub user_name: Option<String>,
    pub created_timestamp: Option<chrono::NaiveDateTime>,
    pub updated_timestamp: Option<chrono::NaiveDateTime>,
}

#[cfg(test)]
impl User {
    pub(crate) fn new(user_name: &str) -> Self {
        User {
            id: None,
            user_name: Some(String::from(user_name)),
            created_timestamp: None,
            updated_timestamp: None,
        }
    }

    pub(crate) fn empty() -> Self {
        User {
            id: None,
            user_name: None,
            created_timestamp: None,
            updated_timestamp: None,
        }
    }
}

#[derive(Clone, Deserialize, Serialize, ToSchema)]
pub struct UserDto {
    pub id: Option<i32>,
    pub user_name: Option<String>,
}

impl AsDtoEnabled<UserDto> for User {
    fn as_dto(&self) -> UserDto {
        UserDto {
            id: self.id,
            user_name: self.user_name.clone(),
        }
    }

    fn from_dto(dto: &UserDto) -> Self {
        Self {
            id: dto.id,
            user_name: dto.user_name.clone(),
            created_timestamp: None,
            updated_timestamp: None,
        }
    }
}

use crate::model::user::{User, UserDto};
use crate::repository::repository_traits::ArcRepository;
use crate::util::AsDtoEnabled;
use axum::http::StatusCode;
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct UserManager {
    user_repository: ArcRepository<User, i32>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum UserError {
    CannotCreateExistingUser,
    MissingId,
    NotFound,
    FailedRequest(String),
}

impl UserManager {
    pub fn new(user_repository: ArcRepository<User, i32>) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, payload: &UserDto) -> Result<UserDto, UserError> {
        if let Some(v) = payload.id {
            error!("Unable to create new user with existing id {v}");
            return Err(UserError::CannotCreateExistingUser);
        }

        info!(
            "Creating new user with username: {}",
            &payload
                .user_name
                .clone()
                .unwrap_or_else(|| "Unknown".to_string())
        );

        let user = User::from_dto(payload);

        self.user_repository
            .create(&user)
            .await
            .map(|u| u.as_dto())
            .ok_or_else(|| UserError::FailedRequest("Failed to create user".to_string()))
    }

    pub async fn update_user(&self, payload: &UserDto) -> Result<UserDto, UserError> {
        if payload.id.is_none() {
            error!("Unable to update a user without an existing id");
            return Err(UserError::MissingId);
        }

        info!("Updating existing user with id: {}", payload.id.unwrap());

        let user = User::from_dto(payload);

        self.user_repository
            .update(&user)
            .await
            .map(|u| u.as_dto())
            .ok_or_else(|| UserError::FailedRequest("Failed to update user".to_string()))
    }

    pub async fn get_user(&self, id: &i32) -> Result<UserDto, UserError> {
        info!("Retrieving user with id: {id}");

        self.user_repository
            .find_by_id(id)
            .await
            .map(|u| u.as_dto())
            .ok_or(UserError::NotFound)
    }

    pub async fn get_users(&self) -> Vec<UserDto> {
        let users = self.user_repository.find_all().await;

        info!("Retrieving {} users", users.len());

        users.iter().map(AsDtoEnabled::as_dto).collect()
    }

    pub async fn delete_user(&self, id: &i32) -> StatusCode {
        info!("Deleting user with id: {id}");

        let res = self.user_repository.delete_by_id(id).await;

        match res {
            0 => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::repository_traits::{ReadRepository, Repository, WriteRepository};
    use async_trait::async_trait;
    use chrono::NaiveDateTime;
    use std::sync::Arc;

    struct MockUserRepository;

    #[async_trait]
    impl ReadRepository<User, i32> for MockUserRepository {
        async fn find_by_id(&self, id: &i32) -> Option<User> {
            match id {
                1 => Some(User::new("foo")),
                _ => None,
            }
        }

        async fn find_all(&self) -> Vec<User> {
            vec![User::new("foo"), User::new("bar"), User::new("baz")]
        }
    }

    #[async_trait]
    impl WriteRepository<User, i32> for MockUserRepository {
        async fn create(&self, entity: &User) -> Option<User> {
            Some(User {
                id: Some(1),
                user_name: entity.user_name.clone(),
                created_timestamp: Some(NaiveDateTime::default()),
                updated_timestamp: Some(NaiveDateTime::default()),
            })
        }

        async fn update(&self, entity: &User) -> Option<User> {
            Some(User {
                id: Some(1),
                user_name: entity.user_name.clone(),
                created_timestamp: Some(NaiveDateTime::default()),
                updated_timestamp: Some(NaiveDateTime::default()),
            })
        }

        async fn delete_by_id(&self, id: &i32) -> u64 {
            match id {
                1 => 1,
                _ => 0,
            }
        }
    }

    impl Repository<User, i32> for MockUserRepository {}

    #[tokio::test]
    async fn test_create_user() {
        let manager = UserManager::new(Arc::new(MockUserRepository));
        let user = UserDto {
            id: None,
            user_name: Some("foo".to_string()),
        };
        let res = manager.create_user(&user).await;

        assert!(res.is_ok());

        let user = res.ok().unwrap();

        assert_eq!(user.id, Some(1));
        assert_eq!(user.user_name, Some("foo".to_string()));
    }

    #[tokio::test]
    async fn test_create_user_with_existing_id() {
        let manager = UserManager::new(Arc::new(MockUserRepository));
        let user = UserDto {
            id: Some(1),
            user_name: None,
        };
        let res = manager.create_user(&user).await;

        assert!(res.is_err());
        assert_eq!(res.err(), Some(UserError::CannotCreateExistingUser))
    }

    #[tokio::test]
    async fn get_user_by_id() {
        let manager = UserManager::new(Arc::new(MockUserRepository));
        let res = manager.get_user(&1).await;

        assert!(res.is_ok());
        assert_eq!(res.ok().unwrap().user_name, Some("foo".to_string()));
    }

    #[tokio::test]
    async fn get_user_by_id_not_found() {
        let manager = UserManager::new(Arc::new(MockUserRepository));
        let res = manager.get_user(&123).await;

        assert!(res.is_err());
        assert_eq!(res.err(), Some(UserError::NotFound));
    }

    #[tokio::test]
    async fn get_all_users() {
        let manager = UserManager::new(Arc::new(MockUserRepository));
        let res = manager.get_users().await;
        let users: Vec<String> = res.iter().map(|u| u.user_name.clone().unwrap()).collect();

        assert_eq!(users, vec!["foo", "bar", "baz"]);
    }

    #[tokio::test]
    async fn test_update_user() {
        let manager = UserManager::new(Arc::new(MockUserRepository));
        let user = UserDto {
            id: Some(1),
            user_name: Some("foo".to_string()),
        };
        let res = manager.update_user(&user).await;

        assert!(res.is_ok());

        let user = res.ok().unwrap();

        assert_eq!(user.id, Some(1));
        assert_eq!(user.user_name, Some("foo".to_string()));
    }

    #[tokio::test]
    async fn test_update_user_with_missing_id() {
        let manager = UserManager::new(Arc::new(MockUserRepository));
        let user = UserDto {
            id: None,
            user_name: None,
        };
        let res = manager.update_user(&user).await;

        assert!(res.is_err());
        assert_eq!(res.err(), Some(UserError::MissingId))
    }

    #[tokio::test]
    async fn test_delete_user() {
        let manager = UserManager::new(Arc::new(MockUserRepository));
        let res = manager.delete_user(&1).await;

        assert_eq!(res, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_missing_user() {
        let manager = UserManager::new(Arc::new(MockUserRepository));
        let res = manager.delete_user(&123).await;

        assert_eq!(res, StatusCode::NOT_FOUND);
    }
}

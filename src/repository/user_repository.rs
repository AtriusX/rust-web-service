use crate::model::user::User;
use crate::repository::repository_traits::{ReadRepository, Repository, WriteRepository};
use async_trait::async_trait;
use sqlx::{query_as, PgPool};

#[derive(Clone)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: &PgPool) -> Self {
        Self { pool: pool.clone() }
    }
}

#[async_trait]
impl ReadRepository<User, i32> for UserRepository {
    async fn find_by_id(&self, id: &i32) -> Option<User> {
        let query = query_as!(
            User,
            "
            select *
            from user_account
            where id = $1
        ",
            &id
        );
        let user = query.fetch_one(&self.pool).await;

        user.ok()
    }

    async fn find_all(&self) -> Vec<User> {
        let query = query_as!(
            User,
            "
            select *
            from user_account
            order by id
        "
        );
        let users = query.fetch_all(&self.pool).await;

        users.unwrap_or(Vec::new())
    }
}

#[async_trait]
impl WriteRepository<User, i32> for UserRepository {
    async fn create(&self, entity: &User) -> Option<User> {
        let query = query_as!(
            User,
            "
            insert into user_account (user_name)
            values ($1)
            returning id, user_name, created_timestamp, updated_timestamp
        ",
            entity.user_name
        );
        let user = query.fetch_one(&self.pool).await;

        user.ok()
    }

    async fn update(&self, entity: &User) -> Option<User> {
        let query = query_as!(
            User,
            "
            update user_account
            set user_name = $1,
                updated_timestamp = now()
            where id = $2
            returning id, user_name, created_timestamp, updated_timestamp
        ",
            entity.user_name,
            entity.id
        );
        let user = query.fetch_one(&self.pool).await;

        user.ok()
    }

    async fn delete_by_id(&self, id: &i32) -> u64 {
        let query = query_as!(
            User,
            "
            delete
            from user_account
            where id = $1
        ",
            &id
        );

        query
            .execute(&self.pool)
            .await
            .map(|r| r.rows_affected())
            .unwrap_or(0)
    }
}

impl Repository<User, i32> for UserRepository {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::repository_traits::TruncateRepository;

    #[async_trait]
    impl TruncateRepository for UserRepository {
        async fn truncate(&self) -> u64 {
            let query = query_as!(
                User,
                "
            truncate user_account
        "
            );

            query
                .execute(&self.pool)
                .await
                .map(|r| r.rows_affected())
                .unwrap_or(0)
        }
    }

    #[sqlx::test]
    async fn test_db_connection(pool: PgPool) {
        assert_eq!(pool.is_closed(), false);
    }

    #[sqlx::test]
    async fn test_create_user(pool: PgPool) {
        let user = User::new("foo");
        let repo = UserRepository::new(&pool);
        let user = repo.create(&user).await.expect("User could not be created");

        assert_eq!(user.id.is_some(), true);
        assert_eq!(user.user_name.unwrap(), "foo");
        assert_eq!(user.created_timestamp.is_some(), true);
        assert_eq!(user.updated_timestamp.is_some(), true);
    }

    #[sqlx::test]
    async fn test_create_bad_user(pool: PgPool) {
        let user = User::empty();
        let repo = UserRepository::new(&pool);
        let res = repo.create(&user).await;

        assert_eq!(res.is_none(), true);
    }

    #[sqlx::test]
    async fn test_get_user_by_id(pool: PgPool) {
        let user = User::new("foo");
        let repo = UserRepository::new(&pool);
        let user = repo.create(&user).await.expect("User could not be created");
        let user = repo
            .find_by_id(&user.id.unwrap())
            .await
            .expect("User wasn't retrieved");

        assert_eq!(user.id.is_some(), true);
        assert_eq!(user.user_name.unwrap(), "foo");
        assert_eq!(user.created_timestamp.is_some(), true);
        assert_eq!(user.updated_timestamp.is_some(), true);
    }

    #[sqlx::test]
    async fn test_get_all_users(pool: PgPool) {
        let users = vec![
            User::new("foobar"),
            User::new("foobaz"),
            User::new("bazbar"),
        ];
        let repo = UserRepository::new(&pool);
        let _ = repo.truncate().await;

        for user in users {
            let _ = repo.create(&user).await;
        }

        let users = repo.find_all().await;

        assert_eq!(users.len(), 3);

        let names: Vec<String> = users.iter().map(|u| u.user_name.clone().unwrap()).collect();

        assert_eq!(names[0], "foobar".to_string());
        assert_eq!(names[1], "foobaz".to_string());
        assert_eq!(names[2], "bazbar".to_string());
    }

    #[sqlx::test]
    async fn test_delete_user_by_id(pool: PgPool) {
        let user = User::new("foo");
        let repo = UserRepository::new(&pool);
        let user = repo.create(&user).await;
        let delete = repo.delete_by_id(&user.unwrap().id.unwrap()).await;

        assert_eq!(delete, 1);
    }

    #[sqlx::test]
    async fn test_delete_missing_user(pool: PgPool) {
        let user = User::new("foo");
        let repo = UserRepository::new(&pool);
        let user = repo.create(&user).await;
        let id = &user.unwrap().id.unwrap();
        let _ = repo.delete_by_id(&id).await;
        let delete = repo.delete_by_id(&id).await;

        assert_eq!(delete, 0);
    }
}

use crate::manager::UserError;
use crate::model::api_response::{ApiResponse, AsApiResponse};
use crate::model::auth_error::AuthError;
use crate::model::user::UserDto;
use crate::state::{AppState, UsersApi};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

const USER_TAG: &str = "User";

pub fn get_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(create_user))
        .routes(routes!(get_user))
        .routes(routes!(update_user))
        .routes(routes!(delete_user))
        .routes(routes!(get_users))
}

#[utoipa::path(
    post,
    path = "/user",
    request_body = UserDto,
    responses(
        (status = 200, description = "New user created", body = UserDto),
        (status = 401, description = "Not authorized", body = AuthError),
        (status = 409, description = "User already exists", body = UserError),
        (status = 500, description = "Failed to save user", body = UserError),
    ),
    tag = USER_TAG,
)]
async fn create_user(
    State(UsersApi { user_manager, .. }): State<UsersApi>,
    Json(payload): Json<UserDto>,
) -> ApiResponse<UserDto, UserError> {
    user_manager
        .create_user(&payload)
        .await
        .as_api_response(StatusCode::CREATED)
}

#[utoipa::path(
    put,
    path = "/user",
    request_body = UserDto,
    responses(
        (status = 200, description = "User updated", body = UserDto),
        (status = 401, description = "Not authorized", body = AuthError),
        (status = 400, description = "Missing request info", body = UserError),
        (status = 500, description = "Failed to find or update user", body = UserError),
    ),
    tag = USER_TAG,
)]
async fn update_user(
    State(UsersApi { user_manager, .. }): State<UsersApi>,
    Json(payload): Json<UserDto>,
) -> ApiResponse<UserDto, UserError> {
    user_manager
        .update_user(&payload)
        .await
        .as_api_response_ok()
}

#[utoipa::path(
    get,
    path = "/user/{id}",
    responses(
        (status = 200, description = "User found", body = UserDto),
        (status = 401, description = "Not authorized", body = AuthError),
        (status = 404, description = "User not found", body = UserError),
    ),
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    tag = USER_TAG,
)]
async fn get_user(
    State(UsersApi { user_manager, .. }): State<UsersApi>,
    Path(id): Path<i32>,
) -> ApiResponse<UserDto, UserError> {
    user_manager
        .get_user(&id)
        .await
        .as_api_response_ok()
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "Users found", body = Vec<UserDto>),
        (status = 401, description = "Not authorized", body = AuthError),
    ),
    tag = USER_TAG,
)]
async fn get_users(
    State(UsersApi { user_manager, .. }): State<UsersApi>,
) -> (StatusCode, Json<Vec<UserDto>>) {
    (StatusCode::OK, Json(user_manager.get_users().await))
}

#[utoipa::path(
    delete,
    path = "/user/{id}",
    responses(
        (status = 200, description = "Users found", body = Vec<UserDto>),
        (status = 401, description = "Not authorized", body = AuthError),
    ),
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    tag = USER_TAG,
)]
async fn delete_user(
    State(UsersApi { user_manager, .. }): State<UsersApi>,
    Path(id): Path<i32>,
) -> (StatusCode, Json<()>) {
    (user_manager.delete_user(&id).await, Json(()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config;
    use axum::body::Body;
    use axum::http::header::CONTENT_TYPE;
    use axum::http::Request;
    use axum::Router;
    use http_body_util::BodyExt;
    use serde_json::Value;
    use sqlx::PgPool;
    use tower::util::ServiceExt;

    async fn create_user(app: &Router, user: UserDto) -> axum::response::Response {
        let req = Request::post("/user")
            .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_string(&user).unwrap()))
            .unwrap();
        app.clone().oneshot(req).await.unwrap()
    }

    async fn update_user(app: &Router, user: UserDto) -> axum::response::Response {
        let req = Request::put("/user")
            .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(serde_json::to_string(&user).unwrap()))
            .unwrap();

        app.clone().oneshot(req).await.unwrap()
    }

    async fn delete_user(app: &Router, id: i32) -> axum::response::Response {
        let req = Request::delete(format!("/user/{id}"))
            .body(Body::empty())
            .unwrap();
        app.clone().oneshot(req).await.unwrap()
    }

    async fn get_user(app: &Router, id: i32) -> axum::response::Response {
        let req = Request::get(format!("/user/{id}"))
            .body(Body::empty())
            .unwrap();
        app.clone().oneshot(req).await.unwrap()
    }

    async fn get_all_users(app: &Router) -> axum::response::Response {
        let req = Request::get("/users").body(Body::empty()).unwrap();
        app.clone().oneshot(req).await.unwrap()
    }

    #[inline]
    async fn unwrap_res(res: axum::response::Response) -> Value {
        let body = res.into_body().collect().await.unwrap().to_bytes();
        serde_json::from_slice(&body).unwrap()
    }

    #[inline]
    async fn unwrap_ok(res: axum::response::Response) -> Value {
        unwrap_res(res).await.clone()
    }

    #[inline]
    async fn unwrap_err(res: axum::response::Response) -> Value {
        unwrap_res(res).await.clone()
    }

    fn user(user_name: &str) -> UserDto {
        UserDto {
            id: None,
            user_name: Some(user_name.to_string()),
        }
    }

    fn existing_user(id: i32, user_name: &str) -> UserDto {
        UserDto {
            id: Some(id),
            user_name: Some(user_name.to_string()),
        }
    }

    async fn app(pool: PgPool) -> Router {
        let routes = vec![get_routes()];

        config::app(pool, vec![], routes).await
    }

    #[sqlx::test]
    async fn test_create_user(pool: PgPool) {
        let app = app(pool).await;
        let user = user("foo");
        let res = create_user(&app, user).await;

        assert!(res.status().is_success());

        let body = unwrap_ok(res).await;

        assert!(body.get("id").is_some());
        assert_eq!(body["user_name"], "foo");
    }

    #[sqlx::test]
    async fn test_create_existing_user(pool: PgPool) {
        let app = app(pool).await;
        let user = existing_user(1, "foo");
        let res = create_user(&app, user).await;

        assert!(res.status().is_client_error());

        let body = unwrap_err(res).await;

        assert_eq!(body, "CannotCreateExistingUser");
    }

    #[sqlx::test]
    async fn test_get_user(pool: PgPool) {
        let user = user("foo");
        let app = app(pool).await;
        let res = create_user(&app, user).await;
        let body = unwrap_ok(res).await;
        let res = get_user(&app, body["id"].as_i64().unwrap() as i32).await;

        assert!(res.status().is_success());

        let body = unwrap_ok(res).await;

        assert_eq!(body["user_name"], "foo");
    }

    #[sqlx::test]
    async fn test_get_missing_user(pool: PgPool) {
        let app = app(pool).await;
        let test_user = 23423423;

        delete_user(&app, test_user).await;

        let res = get_user(&app, test_user).await;

        assert!(res.status().is_client_error());

        let body = unwrap_err(res).await;

        assert_eq!(body, "NotFound");
    }

    #[sqlx::test]
    async fn test_get_all_users(pool: PgPool) {
        let users: Vec<UserDto> = vec!["foo123", "bar123", "baz123"]
            .iter()
            .map(|s| user(s))
            .collect();
        let app = app(pool).await;

        for user in users {
            create_user(&app, user).await;
        }

        let res = get_all_users(&app).await;

        assert!(res.status().is_success());

        let body = unwrap_res(res).await.clone();
        let body = body.as_array().unwrap();

        assert!(body.iter().any(|u| u["user_name"] == "foo123"));
        assert!(body.iter().any(|u| u["user_name"] == "bar123"));
        assert!(body.iter().any(|u| u["user_name"] == "baz123"));
    }

    #[sqlx::test]
    async fn test_update_user(pool: PgPool) {
        let app = app(pool).await;
        let user = user("foo");
        let res = create_user(&app, user).await;

        assert!(res.status().is_success());

        let body = unwrap_ok(res).await;

        assert!(body.get("id").is_some());
        assert_eq!(body.get("user_name").unwrap(), "foo");

        let id = body["id"].as_i64().unwrap() as i32;
        let user = existing_user(id, "bar");
        let res = update_user(&app, user).await;
        let body = unwrap_ok(res).await;

        assert_eq!(body["id"], id);
        assert_eq!(body["user_name"], "bar");
    }

    #[sqlx::test]
    async fn test_update_missing_user(pool: PgPool) {
        let app = app(pool).await;
        let user = user("foo");
        let res = update_user(&app, user).await;

        assert!(res.status().is_client_error());

        let body = unwrap_err(res).await;

        assert_eq!(body, "MissingId");
    }

    #[sqlx::test]
    async fn test_delete_user(pool: PgPool) {
        let app = app(pool).await;
        let user = user("foo");
        let res = create_user(&app, user).await;
        let body = unwrap_ok(res).await;

        assert!(body.get("id").is_some());

        let id = body["id"].as_i64().unwrap() as i32;
        let res = delete_user(&app, id).await;

        assert!(res.status().is_success());
    }

    #[sqlx::test]
    async fn test_delete_missing_user(pool: PgPool) {
        let app = app(pool).await;
        let test_user = 23423423;

        delete_user(&app, test_user).await;

        let res = delete_user(&app, test_user).await;

        assert!(res.status().is_client_error());
    }
}

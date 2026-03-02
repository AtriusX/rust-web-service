use crate::manager::{AuthManager, AuthTokenResponse};
use crate::model::api_response::{ApiError, AsCookieApiResponse, CookieApiResponse};
use crate::model::auth::LoginDto;
use crate::state::AppState;
use axum::extract::State;
use axum::Json;
use axum_extra::extract::CookieJar;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

const AUTH_TAG: &str = "Authorization";

pub fn get_routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(login))
        .routes(routes!(refresh))
        .routes(routes!(logout))
}

#[utoipa::path(
    post,
    path = "/auth/login",
    responses(
        (status = OK, description = "Log in the specified user", body = AuthTokenResponse),
        (status = "default", description = "General API Error", body = ApiError),
    ),
    tag = AUTH_TAG,
    security(),
)]
async fn login(
    jar: CookieJar,
    State(state): State<AuthManager>,
    Json(payload): Json<LoginDto>,
) -> CookieApiResponse<AuthTokenResponse> {
    state.login(jar, payload).await.as_cookie_api_response_ok()
}

#[utoipa::path(
    post,
    path = "/auth/refresh",
    responses(
        (status = OK, description = "Refresh the user's login", body = AuthTokenResponse),
        (status = "default", description = "General API Error", body = ApiError),
    ),
    tag = AUTH_TAG,
    security(),
)]
async fn refresh(
    jar: CookieJar,
    State(state): State<AuthManager>,
) -> CookieApiResponse<AuthTokenResponse> {
    state.refresh(jar).await.as_cookie_api_response_ok()
}

#[utoipa::path(
    post,
    path = "/auth/logout",
    responses(
        (status = OK, description = "Refresh the user's login", body = ()),
        (status = "default", description = "General API Error", body = ApiError),
    ),
    tag = AUTH_TAG,
)]
async fn logout(
    jar: CookieJar,
    State(state): State<AuthManager>,
) -> CookieApiResponse<()> {
    state.logout(&jar).await.as_cookie_api_response_ok()
}

use crate::model::auth::LoginDto;
use crate::model::auth_error::AuthError;
use crate::services::AuthService;
use axum_extra::extract::cookie::{Cookie, SameSite};
use axum_extra::extract::CookieJar;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct AuthTokenResponse {
    access_token: String,
    token_type: String,
}

#[derive(Clone)]
pub struct AuthManager {
    auth_service: AuthService,
}

impl AuthManager {
    pub fn new(auth_service: AuthService) -> Self {
        Self { auth_service }
    }

    pub async fn login(&self, jar: CookieJar, payload: LoginDto) -> Result<(CookieJar, AuthTokenResponse), AuthError> {
        if payload.user_name.is_empty() || payload.password.is_empty() {
            return Err(AuthError::MissingCredentials);
        }

        if payload.user_name != "foo" || payload.password != "bar" {
            return Err(AuthError::WrongCredentials);
        }
        // Check to make sure the user isn't already logged in
        let existing_refresh = &jar.get("refresh_token");
        if existing_refresh.is_some() {
            // Invalidate the old token to prevent dangling sessions
            let _ = self.logout(&jar).await;
        }

        let body = self.auth_service
            .generate_login_tokens(payload.user_name.as_str()).await?;
        let cookie = self.get_cookie(body.refresh_token);
        let jar = jar.add(cookie);
        let token = AuthTokenResponse {
            access_token: body.access_token,
            token_type: "Bearer".to_string(),
        };

        Ok((jar, token))
    }

    pub async fn refresh(&self, jar: CookieJar) -> Result<(CookieJar, AuthTokenResponse), AuthError> {
        let refresh_token = jar.get("refresh_token");
        let refresh_token = match refresh_token {
            Some(refresh_token) => refresh_token,
            None => return Err(AuthError::InvalidToken),
        };
        let refresh_token = match Uuid::parse_str(refresh_token.value()) {
            Ok(v) => v,
            Err(_) => return Err(AuthError::InvalidToken),
        };
        let refresh = self.auth_service
            .refresh_login(refresh_token).await;

        match refresh {
            Err(_) => Err(AuthError::InvalidToken),
            Ok(v) => {
                let res = jar.add(self.get_cookie(v.refresh_token));
                let token = AuthTokenResponse {
                    token_type: "Bearer".to_string(),
                    access_token: v.access_token,
                };
                Ok((res, token))
            }
        }
    }

    pub async fn logout(&self, jar: &CookieJar) -> Result<(CookieJar, ()), AuthError> {
        let refresh_token = jar.get("refresh_token");

        if let Some(refresh_token) = refresh_token {
            let _ = self.auth_service.invalidate_login(refresh_token.value()).await;
        }

        let jar = jar.clone().add(self.remove_refresh_cookie());
        Ok((jar, ()))
    }

    fn get_cookie<'a>(&self, refresh_token: Uuid) -> Cookie<'a> {
        Cookie::build(("refresh_token", refresh_token.to_string()))
            .path("auth/refresh;auth/logout")
            // .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
            .build()
    }

    fn remove_refresh_cookie<'a>(&self) -> Cookie<'a> {
        Cookie::build(("refresh_token", ""))
            .removal()
            .build()
    }
}
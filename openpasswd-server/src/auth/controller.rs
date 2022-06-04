use super::{
    dto::{auth_error::AuthResult, claims::Claims},
    service::AuthService,
};
use crate::{
    auth::dto::refresh_token::{RefreshTokenClaims, REFRESH_TOKEN_COOKIE_NAME},
    core::{cache::Cache, validator::ValidatedJson},
    repository::Repository,
};
use axum::{
    http::{header::SET_COOKIE, HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use log::info;
use openpasswd_model::auth::{
    LoginRequest, PasswordRecoveryFinish, PasswordRecoveryStart, RefreshTokenType, UserRegister,
};

pub async fn token(
    ValidatedJson(login): ValidatedJson<LoginRequest>,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    let mut access_token = auth_service.login(&login).await?;

    let mut headers = HeaderMap::new();
    match login.refresh_token {
        Some(RefreshTokenType::Cookie) => {
            if let Some(refresh_token) = access_token.refresh_token.take() {
                let cookie = cookie::Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_token)
                    .domain("localhost")
                    .path("/")
                    .secure(true)
                    .http_only(true)
                    .max_age(cookie::time::Duration::minutes(5))
                    .finish();

                headers.insert(SET_COOKIE, cookie.to_string().parse().unwrap());
            }
        }
        _ => (),
    }

    Ok((StatusCode::OK, headers, Json(access_token)))
}

pub async fn refresh_token(
    refresh_token: RefreshTokenClaims,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    let mut access_token = auth_service.refresh_token(&refresh_token).await?;

    let mut headers = HeaderMap::new();
    match refresh_token.refresh_token_type {
        RefreshTokenType::Cookie => {
            if let Some(refresh_token) = access_token.refresh_token.take() {
                let cookie = cookie::Cookie::build(REFRESH_TOKEN_COOKIE_NAME, refresh_token)
                    .domain("localhost")
                    .path("/")
                    .secure(true)
                    .http_only(true)
                    .max_age(cookie::time::Duration::minutes(5))
                    .finish();

                headers.insert(SET_COOKIE, cookie.to_string().parse().unwrap());
            }
        }
        _ => (),
    }

    Ok((StatusCode::OK, headers, Json(access_token)))
}

pub async fn logout(
    claims: Claims,
    refresh_token: Option<RefreshTokenClaims>,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    auth_service.logout(claims, refresh_token).await?;
    Ok(StatusCode::OK)
}

pub async fn register(
    ValidatedJson(user): ValidatedJson<UserRegister>,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    auth_service.register(user).await?;
    Ok(StatusCode::CREATED)
}

pub async fn get_me(
    claims: Claims,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    let user = auth_service.get_me(claims.sub).await?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn password_recovery_start(
    ValidatedJson(pass_recovery): ValidatedJson<PasswordRecoveryStart>,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    auth_service.password_recovery_start(pass_recovery).await?;
    Ok(StatusCode::CREATED.into_response())
}

pub async fn password_recovery_finish(
    ValidatedJson(pass_recovery): ValidatedJson<PasswordRecoveryFinish>,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    auth_service.password_recovery_finish(pass_recovery).await?;
    Ok(StatusCode::OK.into_response())
}

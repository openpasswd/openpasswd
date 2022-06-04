use super::{
    dto::{auth_error::AuthResult, claims::Claims},
    service::AuthService,
};
use crate::{
    auth::dto::refresh_token::{RefreshTokenRequest, REFRESH_TOKEN_COOKIE_NAME},
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
    LoginRequest, PasswordRecoveryFinish, PasswordRecoveryStart, UserRegister,
};

pub async fn token(
    ValidatedJson(login): ValidatedJson<LoginRequest>,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    let access_token = auth_service.login(&login).await?;

    let cookie = cookie::Cookie::build(REFRESH_TOKEN_COOKIE_NAME, uuid::Uuid::new_v4().to_string())
        .domain("localhost")
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(cookie::time::Duration::hours(1))
        .finish();

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.to_string().parse().unwrap());

    Ok((StatusCode::OK, headers, Json(access_token)))
}

pub async fn refresh_token(refresh_token: RefreshTokenRequest) -> AuthResult<impl IntoResponse> {
    info!("{:?}", refresh_token);

    Ok(StatusCode::OK)
}

pub async fn logout(
    claims: Claims,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    auth_service.logout(claims).await?;
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

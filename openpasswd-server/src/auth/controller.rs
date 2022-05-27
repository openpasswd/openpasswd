use super::{
    dto::{auth_error::AuthResult, claims::Claims},
    service::AuthService,
};
use crate::{
    core::{cache::Cache, validator::ValidatedJson},
    repository::Repository,
};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use openpasswd_model::auth::{LoginRequest, PasswordRecovery, UserRegister};

pub async fn token(
    ValidatedJson(login): ValidatedJson<LoginRequest>,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    let access_token = auth_service.login(&login).await?;
    Ok((StatusCode::OK, Json(access_token)))
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
    ValidatedJson(pass_recovery): ValidatedJson<PasswordRecovery>,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    auth_service.password_recovery_start(&pass_recovery).await?;
    Ok(StatusCode::CREATED.into_response())
}

pub async fn password_recovery_finish(
    ValidatedJson(pass_recovery): ValidatedJson<PasswordRecovery>,
    Extension(repository): Extension<Repository>,
    Extension(cache): Extension<Cache>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(repository, cache);
    auth_service
        .password_recovery_finish(&pass_recovery)
        .await?;
    Ok(StatusCode::CREATED.into_response())
}

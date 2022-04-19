use super::{
    dto::{auth_error::AuthResult, claims::Claims},
    service::AuthService,
};
use crate::{core::validator::ValidatedJson, DynPgConnection};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use openpasswd_model::auth::{AccessToken, LoginRequest, UserRegister};

pub async fn token(
    ValidatedJson(login): ValidatedJson<LoginRequest>,
    Extension(connection): Extension<DynPgConnection>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(connection);
    let access_token = auth_service.login(&login)?;

    let token = AccessToken {
        access_token,
        token_type: String::from("Bearer"),
    };

    Ok((StatusCode::OK, Json(token)))
}

pub async fn register(
    ValidatedJson(user): ValidatedJson<UserRegister>,
    Extension(connection): Extension<DynPgConnection>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(connection);
    auth_service.register(&user)?;
    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_me(
    claims: Claims,
    Extension(connection): Extension<DynPgConnection>,
) -> AuthResult<impl IntoResponse> {
    let auth_service = AuthService::new(connection);
    let user = auth_service.get_me(&claims.sub)?;

    Ok((StatusCode::OK, Json(user)))
}

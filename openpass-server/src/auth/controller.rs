use super::dto::{auth_error::AuthResult, claims::Claims};
use crate::{core::validator::ValidatedJson, DynPgConnection};
use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use openpass_model::auth::{AccessToken, LoginRequest, UserRegister};

pub async fn token(
    ValidatedJson(login): ValidatedJson<LoginRequest>,
    Extension(connection): Extension<DynPgConnection>,
) -> AuthResult<impl IntoResponse> {
    let access_token = super::service::login(&login, connection)?;

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
    super::service::register(&user, connection)?;
    Ok(StatusCode::CREATED.into_response())
}

pub async fn get_me(
    claims: Claims,
    Extension(connection): Extension<DynPgConnection>,
) -> AuthResult<impl IntoResponse> {
    Ok(StatusCode::OK)
    // if let Err(s) = super::service::register(&user, connection) {
    //     (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: s })).into_response()
    // } else {
    //     StatusCode::CREATED.into_response()
    // }
}

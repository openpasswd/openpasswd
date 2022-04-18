use crate::{core::validator::ValidatedJson, DynPgConnection};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use openpass_model::{
    auth::{AccessToken, LoginRequest, UserRegister},
    error::ErrorResponse,
};

use super::dto::Claims;

pub async fn token(
    ValidatedJson(login): ValidatedJson<LoginRequest>,
    Extension(connection): Extension<DynPgConnection>,
) -> impl IntoResponse {
    match super::service::login(&login, connection) {
        Ok(access_token) => {
            let token = AccessToken {
                access_token,
                token_type: String::from("Bearer"),
            };

            (StatusCode::OK, Json(token)).into_response()
        }
        Err(s) => (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: s })).into_response(),
    }
}

pub async fn register(
    ValidatedJson(user): ValidatedJson<UserRegister>,
    Extension(connection): Extension<DynPgConnection>,
) -> Response {
    if let Err(s) = super::service::register(&user, connection) {
        (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: s })).into_response()
    } else {
        StatusCode::CREATED.into_response()
    }
}

pub async fn get_me(claims: Claims, Extension(connection): Extension<DynPgConnection>) -> Response {
    todo!()
    // if let Err(s) = super::service::register(&user, connection) {
    //     (StatusCode::BAD_REQUEST, Json(ErrorResponse { error: s })).into_response()
    // } else {
    //     StatusCode::CREATED.into_response()
    // }
}

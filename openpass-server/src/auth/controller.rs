use crate::{
    core::validator::ValidatedJson,
    dto::{AccessToken, UserRegister},
    DynPgConnection,
};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use openpass_model::error::ErrorResponse;

pub async fn token(ValidatedJson(user): ValidatedJson<UserRegister>) -> impl IntoResponse {
    let token = AccessToken {
        access_token: String::from("xxx"),
        token_type: String::from("Bearer"),
    };

    (StatusCode::OK, Json(token))
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

use std::collections::HashMap;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use model::error::ErrorResponse;

pub type AccountResult<T = ()> = Result<T, AccountError>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AccountError {
    InvalidAccountGroup,
    NotFound,
}

impl IntoResponse for AccountError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AccountError::InvalidAccountGroup => (
                StatusCode::BAD_REQUEST,
                String::from("Invalid Account Group"),
            ),
            AccountError::NotFound => (StatusCode::NOT_FOUND, String::from("Invalid Path")),
        };
        let body = Json(ErrorResponse {
            error: HashMap::from([(String::from("message"), error_message)]),
        });
        (status, body).into_response()
    }
}

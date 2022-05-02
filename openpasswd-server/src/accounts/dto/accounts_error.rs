use std::collections::HashMap;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use openpasswd_model::error::ErrorResponse;

pub type AccountResult<T = ()> = Result<T, AccountError>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AccountError {
    InvalidAccountGroup,
}

impl IntoResponse for AccountError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AccountError::InvalidAccountGroup => (
                StatusCode::BAD_REQUEST,
                String::from("Invalid Account Group"),
            ),
        };
        let body = Json(ErrorResponse {
            error: HashMap::from([(String::from("message"), error_message)]),
        });
        (status, body).into_response()
    }
}

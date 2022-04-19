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
    Sample,
}

impl IntoResponse for AccountError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AccountError::Sample => (StatusCode::BAD_REQUEST, String::from("Sample Error")),
        };
        let body = Json(ErrorResponse {
            error: error_message,
        });
        (status, body).into_response()
    }
}

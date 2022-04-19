use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use openpasswd_model::error::ErrorResponse;

pub type AuthResult<T = ()> = Result<T, AuthError>;

#[allow(dead_code)]
#[derive(Debug)]
pub enum AuthError {
    // Authorization
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    // Authentication
    InvalidCredentials,
    JwtEncode(String),
    // Create
    EmailAlreadyTaken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            // Authorization
            AuthError::WrongCredentials => {
                (StatusCode::UNAUTHORIZED, String::from("Wrong credentials"))
            }
            AuthError::MissingCredentials => {
                (StatusCode::BAD_REQUEST, String::from("Missing credentials"))
            }
            AuthError::TokenCreation => (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("Token creation error"),
            ),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, String::from("Invalid token")),
            // Authentication
            AuthError::InvalidCredentials => (
                StatusCode::BAD_REQUEST,
                String::from("Email or password is incorrect"),
            ),
            AuthError::JwtEncode(e) => (StatusCode::BAD_REQUEST, e),
            // Create
            AuthError::EmailAlreadyTaken => (
                StatusCode::BAD_REQUEST,
                String::from("Email already in use"),
            ),
        };
        let body = Json(ErrorResponse {
            error: error_message,
        });
        (status, body).into_response()
    }
}

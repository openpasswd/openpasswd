use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Debug)]
// #[serde(rename_all = "snake_case")]
pub enum RefreshTokenType {
    #[serde(alias = "Cookie", alias = "cookie")]
    Cookie,
    #[serde(alias = "Token", alias = "token")]
    Token,
}

#[derive(Deserialize, Validate, Debug)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is invalid"))]
    pub password: String,
    pub device_name: Option<String>,
    pub refresh_token: Option<RefreshTokenType>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UserRegister {
    #[validate(length(min = 1, message = "Name is invalid"))]
    pub name: String,
    #[validate(length(min = 1, message = "Email is invalid"))]
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 1, message = "Password is invalid"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AccessToken {
    pub access_token: String,
    #[serde(rename = "type")]
    pub token_type: String,
    pub refresh_token: Option<String>,
}

#[derive(Serialize)]
pub struct UserView {
    pub name: String,
    pub email: String,
    pub last_login: Option<String>,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PasswordRecoveryStart {
    #[validate(length(min = 1, message = "Email is invalid"))]
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct PasswordRecoveryFinish {
    #[validate(length(min = 1, message = "Token is invalid"))]
    pub token: String,

    #[validate(length(min = 1, message = "Password is invalid"))]
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RefreshToken {
    pub refresh_token: String,
}

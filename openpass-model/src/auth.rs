use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct UserRegister {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(email(message = "Email is invalid"))]
    pub email: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct AccessToken {
    pub access_token: String,
    #[serde(rename = "type")]
    pub token_type: String,
}

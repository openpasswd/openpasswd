use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct AccountGroupRegister {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
}

#[derive(Serialize)]
pub struct AccountGroupView {
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct AccountRegister {
    #[validate(length(min = 1, message = "Can not be empty"))]
    pub name: String,
    pub group_id: i32,
    pub level: Option<i16>,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AccountView {
    pub id: String,
    pub name: String,
    pub username: String,
    pub password: Option<String>,
}

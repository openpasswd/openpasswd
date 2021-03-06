use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct AccountGroupRegister {
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountGroupView {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct AccountRegister {
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(range(min = 1))]
    pub group_id: i32,
    pub level: Option<i16>,
    #[validate(length(min = 1))]
    pub username: String,
    #[validate(length(min = 1))]
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountView {
    pub id: i32,
    pub name: String,
    pub group_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct AccountWithPasswordView {
    pub id: i32,
    pub name: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

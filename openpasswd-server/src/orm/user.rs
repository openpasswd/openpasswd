use std::time::SystemTime;

use crate::orm::schema::users;

#[derive(Queryable, Identifiable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub last_login: Option<SystemTime>,
    pub fail_attempts: i16,
    pub last_attempt: Option<SystemTime>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}

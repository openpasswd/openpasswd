use std::time::SystemTime;

use crate::repository::schema::account_groups;
use crate::repository::schema::account_passwords;
use crate::repository::schema::accounts;

#[derive(Queryable, Identifiable)]
pub struct AccountGroup {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
}

#[derive(Queryable, Identifiable)]
pub struct Account {
    pub id: i32,
    pub user_id: i32,
    pub account_groups_id: i32,
    pub level: i16,
    pub name: String,
}

pub struct AccountWithPassword {
    pub id: i32,
    pub user_id: i32,
    pub account_groups_id: i32,
    pub level: i16,
    pub name: String,
    pub passwords: Vec<AccountPassword>,
}

#[derive(Queryable, Identifiable)]
pub struct AccountPassword {
    pub id: i32,
    pub account_id: i32,
    pub username: String,
    pub password: Vec<u8>,
    pub created_date: SystemTime,
}

#[derive(Insertable)]
#[table_name = "account_groups"]
pub struct NewAccountGroup<'a> {
    pub user_id: i32,
    pub name: &'a str,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub user_id: i32,
    pub name: &'a str,
    pub level: Option<i16>,
    pub account_groups_id: i32,
}

#[derive(Insertable)]
#[table_name = "account_passwords"]
pub struct NewAccountPassword<'a> {
    pub account_id: i32,
    pub username: &'a str,
    pub password: &'a [u8],
    pub created_date: SystemTime,
}

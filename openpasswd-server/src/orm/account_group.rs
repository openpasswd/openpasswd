use crate::orm::schema::account_groups;
use crate::orm::schema::accounts;

#[derive(Queryable, Identifiable)]
pub struct AccountGroup {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "account_groups"]
pub struct NewAccountGroup<'a> {
    pub name: &'a str,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "accounts"]
pub struct NewAccount<'a> {
    pub name: &'a str,
    pub level: Option<i16>,
    pub username: &'a str,
    pub password: &'a str,
    pub account_groups_id: i32,
    pub user_id: i32,
}

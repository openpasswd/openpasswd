use chrono::{DateTime, Utc};

pub struct NewAccountGroup {
    pub user_id: i32,
    pub name: String,
}

pub struct NewAccount {
    pub user_id: i32,
    pub name: String,
    pub level: Option<i16>,
    pub account_groups_id: i32,
}

pub struct NewAccountPassword {
    pub account_id: i32,
    pub username: String,
    pub password: Vec<u8>,
    pub created_date: DateTime<Utc>,
}

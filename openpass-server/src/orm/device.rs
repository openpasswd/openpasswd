use std::time::SystemTime;

use crate::orm::schema::devices;

#[derive(Queryable, Identifiable)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub last_access: SystemTime,
    pub active: bool,
    pub public_key: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[table_name = "devices"]
pub struct NewDevice<'a> {
    pub name: &'a str,
    pub last_access: SystemTime,
    pub active: bool,
    pub public_key: &'a str,
    pub user_id: i32,
}

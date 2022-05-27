use std::time::SystemTime;

pub struct Device {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub last_access: SystemTime,
    pub active: bool,
    pub public_key: String,
}

// #[derive(Insertable)]
// #[table_name = "devices"]
// pub struct NewDevice<'a> {
//     pub user_id: i32,
//     pub name: &'a str,
//     pub last_access: SystemTime,
//     pub active: bool,
//     pub public_key: &'a str,
// }

use serde::Serialize;

#[derive(Serialize)]
pub struct Device {
    pub id: String,
    pub name: String,
    pub last_access: String,
    pub active: bool,
}

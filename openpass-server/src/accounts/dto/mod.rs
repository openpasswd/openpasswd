use serde::Serialize;

#[derive(Serialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub username: String,
    pub password: Option<String>,
}

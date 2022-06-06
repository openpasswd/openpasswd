use serde::Serialize;

pub mod accounts;
pub mod auth;
pub mod error;

#[derive(Serialize)]
pub struct List<T> {
    pub items: Vec<T>,
    pub total: u32,
}

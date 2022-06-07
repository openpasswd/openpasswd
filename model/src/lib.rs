use serde::{Deserialize, Serialize};

pub mod accounts;
pub mod auth;
pub mod error;

#[derive(Serialize, Deserialize)]
pub struct List<T> {
    pub items: Vec<T>,
    pub total: u32,
}

use axum::Router;
pub mod controller;
pub mod dto;
mod service;

pub fn route() -> Router {
    Router::new()
    // .route("/api/accounts", get(accounts::list))
    // .route("/api/accounts/:id", get(accounts::get))
}

use axum::{routing::get, Router};

pub mod controller;
pub mod dto;
mod service;

pub fn route() -> Router {
    Router::new()
        .route(
            "/api/accounts",
            get(self::controller::list_accounts).post(self::controller::register_account),
        )
        .route(
            "/api/accounts/groups",
            get(self::controller::list_groups).post(self::controller::register_group),
        )
    // .route("/api/accounts/:id", get(accounts::get))
}

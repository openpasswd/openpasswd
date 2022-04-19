use axum::{
    routing::{get, post},
    Router,
};

pub mod controller;
pub mod dto;
mod service;

pub fn route() -> Router {
    Router::new()
        .route(
            "/api/auth/user",
            get(self::controller::get_me).post(self::controller::register),
        )
        .route("/api/auth/token", post(self::controller::token))
}

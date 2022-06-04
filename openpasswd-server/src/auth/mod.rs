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
        .route(
            "/api/auth/refresh_token",
            post(self::controller::refresh_token),
        )
        .route("/api/auth/logout", post(self::controller::logout))
        .route(
            "/api/auth/password_recovery",
            post(self::controller::password_recovery_start)
                .put(self::controller::password_recovery_finish),
        )
}

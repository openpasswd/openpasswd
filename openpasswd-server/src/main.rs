#[macro_use]
extern crate diesel;

use crate::repository::Repository;
use axum::{
    handler::Handler, http::StatusCode, response::IntoResponse, routing::get, Extension, Router,
};
use dotenvy::dotenv;
use std::net::SocketAddr;

mod accounts;
mod auth;
mod core;
mod devices;
mod repository;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let repository = Repository::new();

    let app = Router::new()
        .merge(root())
        .merge(auth::route())
        .merge(accounts::route())
        .merge(devices::route())
        .layer(Extension(repository))
        .fallback(handler_404.into_service());

    let addr = SocketAddr::from(([0, 0, 0, 0], 7777));
    log::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn root() -> Router {
    Router::new().route("/", get(health_check))
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Health Check")
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::repository::Repository;
use axum::{
    handler::Handler,
    http::{HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::get,
    Extension, Router,
};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};

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
    repository.migration_run();

    let mut app = Router::new()
        .merge(root())
        .merge(auth::route())
        .merge(accounts::route())
        .merge(devices::route())
        .layer(Extension(repository))
        .fallback(handler_404.into_service());

    if let Ok(allow_origin) = std::env::var("CORS_ALLOW_ORIGIN") {
        let mut cors = CorsLayer::new().allow_headers(Any).allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::DELETE,
        ]);

        if allow_origin == "*" {
            cors = cors.allow_origin(Any);
        } else {
            cors = cors.allow_origin(allow_origin.parse::<HeaderValue>().unwrap())
        }

        app = app.layer(cors);
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], 7777));
    log::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
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

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}

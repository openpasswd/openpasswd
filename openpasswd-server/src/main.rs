#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::{
    core::cache::Cache, core::mail_service::EmailAddress, core::mail_service::MailService,
    repository::Repository,
};
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

    let cache = Cache::new().unwrap();

    let mut app = Router::new()
        .merge(root())
        .merge(auth::route())
        .merge(accounts::route())
        .merge(devices::route())
        .layer(Extension(repository))
        .layer(Extension(cache))
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
    Router::new()
        .route("/", get(health_check))
        .route("/cache", get(cache_check))
        .route("/mail", get(mail_check))
}

async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "Health Check")
}

async fn cache_check(Extension(cache): Extension<Cache>) -> impl IntoResponse {
    let key = uuid::Uuid::new_v4().to_string();
    let created_date = chrono::Utc::now();
    let value = created_date.to_rfc3339();
    cache.set_and_expire(&key, value, 15).await;

    let result: String = cache.get(&key).await.unwrap();

    (StatusCode::OK, result)
}

async fn mail_check() -> impl IntoResponse {
    match MailService::send_email(
        EmailAddress::new(Some("nobody"), "nobody@domain.tld"),
        EmailAddress::new(Some("nobody"), "nobody@domain.tld"),
        String::from("Mail Check"),
        core::mail_service::MessageBody::Text(String::from("Hello world")),
    )
    .await
    {
        Ok(()) => StatusCode::OK,
        Err(e) => {
            log::error!("{:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
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

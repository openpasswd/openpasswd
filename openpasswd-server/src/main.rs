#[macro_use]
extern crate diesel;

use axum::{
    handler::Handler, http::StatusCode, response::IntoResponse, routing::get, Extension, Router,
};
use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

mod accounts;
mod auth;
mod core;
mod devices;
mod orm;

type DynPgConnection = Arc<Mutex<PgConnection>>;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = diesel::PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    let arc_connection = Arc::new(Mutex::new(connection)) as DynPgConnection;
    let app = Router::new()
        .merge(root())
        .merge(auth::route())
        .merge(accounts::route())
        .merge(devices::route())
        .layer(Extension(arc_connection))
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

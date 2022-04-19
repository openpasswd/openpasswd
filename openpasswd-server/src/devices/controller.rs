use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use super::dto::Device;

#[derive(Serialize)]
struct List<T> {
    items: Vec<T>,
    total: u32,
}

pub async fn list() -> impl IntoResponse {
    let list = List {
        items: vec![Device {
            id: String::from("47523942-bc63-11ec-8422-0242ac120002"),
            name: String::from("netflix"),
            last_access: chrono::Local::now().to_string(),
            active: true,
        }],
        total: 1,
    };

    (StatusCode::OK, Json(list))
}

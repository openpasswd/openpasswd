use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse<T> {
    pub error: T,
}

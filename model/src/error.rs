use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse<T> {
    pub error: HashMap<String, T>,
}

use std::{collections::HashMap, fmt::format};

use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    response::{IntoResponse, Response},
    BoxError, Json,
};
use openpass_model::error::ErrorResponse;
use serde::de::DeserializeOwned;
use validator::Validate;

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: axum::body::HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req)
            .await
            .map_err(ServerError::AxumJsonRejection)?;
        value.validate().map_err(ServerError::ValidationError)?;
        Ok(ValidatedJson(value))
    }
}

#[derive(Debug)]
pub enum ServerError {
    ValidationError(validator::ValidationErrors),
    AxumJsonRejection(axum::extract::rejection::JsonRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                let mut message = HashMap::new();
                for (field, validation_error) in e.field_errors() {
                    for validation_error_item in validation_error {
                        let value = match validation_error_item.message.as_ref() {
                            Some(x) => x.to_string().replace('"', ""),
                            None => String::from(format!("Field {field} has an invalid value")),
                        };

                        message.insert(field.to_owned(), value);
                    }
                }
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse { error: message }),
                )
                    .into_response()
            }
            ServerError::AxumJsonRejection(e) => {
                let message = match e {
                    axum::extract::rejection::JsonRejection::MissingJsonContentType(_) => {
                        String::from("Expected request with `Content-Type: application/json`")
                    }
                    _ => String::from("Invalid JSON format"),
                };
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse { error: message }),
                )
                    .into_response()
            }
        }
    }
}

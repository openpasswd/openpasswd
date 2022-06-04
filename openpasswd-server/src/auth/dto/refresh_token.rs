use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::Cookie,
    BoxError, Json,
};
use serde::Deserialize;

use super::auth_error::AuthError;

#[derive(Deserialize, Debug)]
pub struct RefreshTokenRequest(pub String);

pub const REFRESH_TOKEN_COOKIE_NAME: &str = "REFRESH_TOKEN";

#[async_trait]
impl<B> FromRequest<B> for RefreshTokenRequest
where
    B: Send,
    B: axum::body::HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();

        let refresh_token = cookie
            .as_ref()
            .and_then(|cookie| cookie.get(REFRESH_TOKEN_COOKIE_NAME));

        if let Some(refresh_token) = refresh_token {
            Ok(RefreshTokenRequest(refresh_token.to_owned()))
        } else {
            let Json(value) = Json::<openpasswd_model::auth::RefreshToken>::from_request(req)
                .await
                .map_err(|_| AuthError::MissingCredentials)?;

            Ok(RefreshTokenRequest(value.refresh_token))
        }
    }
}

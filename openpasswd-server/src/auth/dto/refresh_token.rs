use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::Cookie,
    BoxError, Json,
};
use openpasswd_model::auth::RefreshTokenType;
use serde::{Deserialize, Serialize};

use crate::core::cache::Cache;

use super::auth_error::AuthError;

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshTokenClaims {
    pub jti: String,
    pub sub: i32,
    pub device: Option<String>,
    pub exp: i64,
    pub refresh_token_type: RefreshTokenType,
}

pub const REFRESH_TOKEN_COOKIE_NAME: &str = "REFRESH_TOKEN";

#[async_trait]
impl<B> FromRequest<B> for RefreshTokenClaims
where
    B: Send,
    B: axum::body::HttpBody + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cache = req
            .extensions()
            .get::<Cache>()
            .ok_or(AuthError::MissingStorage)?
            .clone();

        let cookie = Option::<TypedHeader<Cookie>>::from_request(req)
            .await
            .unwrap();

        let refresh_token = cookie
            .as_ref()
            .and_then(|cookie| cookie.get(REFRESH_TOKEN_COOKIE_NAME));

        let (token, refresh_token_type) = if let Some(refresh_token) = refresh_token {
            (refresh_token.to_owned(), RefreshTokenType::Cookie)
        } else {
            let Json(value) = Json::<openpasswd_model::auth::RefreshToken>::from_request(req)
                .await
                .map_err(|_| AuthError::MissingCredentials)?;

            (value.refresh_token, RefreshTokenType::Token)
        };

        let secret =
            std::env::var("JWT_REFRES_TOKEN_SECRET").expect("JWT_REFRES_TOKEN_SECRET must be set");
        let token_data = jsonwebtoken::decode::<RefreshTokenClaims>(
            &token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        if token_data.claims.refresh_token_type != refresh_token_type {
            return Err(AuthError::InvalidToken);
        }

        let key = format!(
            "refresh_token:{}:{}",
            token_data.claims.sub, token_data.claims.jti
        );

        match cache.get::<i32>(&key).await {
            Some(valid_token) if valid_token == 1 => {
                cache.set_keepttl(&key, 0).await;
                Ok(token_data.claims)
            }
            Some(_) => Err(AuthError::InvalidToken),
            None => Err(AuthError::InvalidToken),
        }
    }
}

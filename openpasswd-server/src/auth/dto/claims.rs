use axum::{
    async_trait,
    extract::{FromRequest, RequestParts, TypedHeader},
    headers::{authorization::Bearer, Authorization},
};

use crate::core::cache::Cache;

use super::auth_error::AuthError;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub jti: String,
    pub sub: i32,
    pub device: Option<String>,
    pub exp: i64,
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let cache = req
            .extensions()
            .get::<Cache>()
            .ok_or(AuthError::MissingStorage)?
            .clone();

        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|_| AuthError::MissingCredentials)?;

        let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let token = bearer.token();
        let token_data = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS512),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        let key = format!(
            "signed_token:{}:{}",
            token_data.claims.sub, token_data.claims.jti
        );

        match cache.get::<i32>(&key).await {
            Some(valid_token) if valid_token == 1 => Ok(token_data.claims),
            Some(_) => Err(AuthError::InvalidToken),
            None => Err(AuthError::InvalidToken),
        }
    }
}

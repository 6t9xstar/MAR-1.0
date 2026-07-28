use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use axum::RequestPartsExt;
use axum_extra::headers::authorization::{Authorization, Bearer};
use axum_extra::TypedHeader;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::JwtClaims;
use crate::AppState;
use crate::SharedState;

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub username: String,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = SharedState::from_ref(state);

        let auth = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AppError::Unauthorized("Missing or invalid authorization header".into()))?;

        let token = auth.token();
        let claims = validate_token(token, &state.config.auth.jwt_secret)?;

        Ok(AuthenticatedUser {
            id: claims.sub,
            username: claims.username,
        })
    }
}

impl IntoResponse for AuthenticatedUser {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(json!({ "user_id": self.id, "username": self.username }))).into_response()
    }
}

pub fn validate_token(token: &str, secret: &str) -> Result<JwtClaims, AppError> {
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::default();

    let token_data = decode::<JwtClaims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}

pub fn create_token(claims: &JwtClaims, secret: &str) -> Result<String, AppError> {
    let encoding_key = jsonwebtoken::EncodingKey::from_secret(secret.as_bytes());
    jsonwebtoken::encode(&jsonwebtoken::Header::default(), claims, &encoding_key)
        .map_err(|e| AppError::Internal(format!("Token creation failed: {e}")))
}

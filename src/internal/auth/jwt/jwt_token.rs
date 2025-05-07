use std::future::Future;

use super::jwt_dto::{TokenPayload, WsJwtParams};

use axum::{
    extract::{FromRequestParts, Query},
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{DecodingKey, Validation, decode};

pub struct AuthData {
    pub token_payload: TokenPayload,
    pub group_id: String,
}

pub struct JwtExtractor(pub AuthData);

// S type is Axum State type
impl<S> FromRequestParts<S> for JwtExtractor
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            // get group_id from query params
            let Query(params) =  Query::<WsJwtParams>::from_request_parts(parts, state)
                .await
                .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid query params"))?;

            // get token from header
            let auth_header = parts.headers.get("Authorization")
                .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?
                .to_str()
                .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid Authorization header"))?;

            if !auth_header.starts_with("Bearer ") {
                return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization header"));
            }

            let token = &auth_header[7..];

            // verify token
            let secret = match std::env::var("JWT_SECRET") {
                Ok(val) => val,
                Err(_) => {
                    tracing::warn!("JWT_SECRET not set in environment, using default value (not recommended for production)");
                    "default_secret_key_for_development_only".to_string()
                }
            };

            // log for debug
            tracing::info!("Attempting to decode token");

            let token_data = decode::<TokenPayload>(
                token,
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::new(jsonwebtoken::Algorithm::HS512)
            )
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token"))?;

            Ok(JwtExtractor(AuthData {
                token_payload: token_data.claims,
                group_id: params.group_id,
            }))
        }
    }
}

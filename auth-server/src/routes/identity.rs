use std::sync::Arc;
use anyhow::Context;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use axum::response::IntoResponse;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use log::{error, info, trace};
use serde::{Deserialize, Serialize};
use crate::{AppState};
use crate::error::AppError;
use crate::shared::{SpacetimeClaims, UserClaims};
use crate::tools::validate::validate_user_token;

#[derive(Serialize, Deserialize)]
pub struct IdentityRequest {
    token: String,
}

#[derive(Serialize, Deserialize)]
pub struct IdentityResponse {
    token: String,
    identity: String,
}

pub async fn identity(
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    info!("entry");
    let auth_token = headers.get("authorization").expect("No authorization header").to_str()?;
    let token = auth_token.strip_prefix("Bearer ")
        .unwrap_or(auth_token); 
    info!("token: {}", token);
    let claims = match validate_user_token(token) {
        Ok(claims) => claims,
        Err(err) => {
            error!("{}", err);
            return Err(AppError(anyhow::anyhow!(err)));
        }
    };
    info!("derive_identity_from_claims");
    let identity = derive_identity_from_claims(&claims); // consistent per user
    info!("derive_identity_from_claims");
    let token = issue_spacetimedb_token(&claims, identity.clone())?;

    info!("Returning identity game token");
    Ok(Json(IdentityResponse {
        token,
        identity,
    }))
}
// make SpacetimeClaims.aud: &'a [&'a str], you could avoid a heap allocation if you used:
fn issue_spacetimedb_token(user_claims: &UserClaims, identity: String) -> Result<String, AppError> {
    let now = Utc::now().timestamp();

    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = SpacetimeClaims {
        sub: user_claims.sub.to_string(),
        iss: std::env::var("JWT_ISSUER_SPACETIMEDB").expect("Missing JWT_ISSUER_SPACETIMEDB environment variable."),
        aud: vec!["spacetimedb".to_string()],
        iat: user_claims.iat as usize,
        exp: expiration,
        identity,
    };

    let secret = std::env::var("SECRET_KEY").expect("AUTH_SECRET not set");
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    ).map_err(|e| AppError(anyhow::anyhow!("JWT encoding failed: {}", e)))?;

    Ok(token)
}

fn derive_identity_from_claims(claims: &UserClaims) -> String {
    let input = claims.sub.as_bytes(); // typically the UUID or user ID
    let hash = sha256::digest(input);
    hex::encode(hash)
}


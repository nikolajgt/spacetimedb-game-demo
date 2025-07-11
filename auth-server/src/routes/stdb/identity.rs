use axum::http::HeaderMap;
use axum::Json;
use axum::response::IntoResponse;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use log::{error, info, trace};
use rsa::signature::digest::Digest;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use tokio::fs;
use crate::error::AppError;
use crate::shared::{SpacetimeClaims, UserClaims};
use crate::tools::jwk_builder::compute_kid;
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
    let auth_token = headers.get("authorization").expect("No authorization header").to_str()?;
    let token = auth_token.strip_prefix("Bearer ")
        .unwrap_or(auth_token);
    let claims = match validate_user_token(token) {
        Ok(claims) => claims,
        Err(err) => {
            error!("{}", err);
            return Err(AppError(anyhow::anyhow!(err)));
        }
    };
    let identity = derive_identity_from_claims(&claims);
    let token = issue_spacetimedb_token(&claims, identity.clone()).await?;

    info!("Returning identity game token");
    Ok(Json(IdentityResponse {
        token,
        identity,
    }))
}
// make SpacetimeClaims.aud: &'a [&'a str], you could avoid a heap allocation if you used:
pub async fn issue_spacetimedb_token(
    user_claims: &UserClaims,
    identity: String,
) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let audience = std::env::var("STDB_JWT_AUDIENCE")
        .expect("Missing STDB_JWT_AUDIENCE env var");
    let issuer = std::env::var("STDB_JWT_ISSUER")
        .expect("Missing STDB_JWT_ISSUER env var");

    let claims = SpacetimeClaims {
        sub: user_claims.sub.to_string(),
        iss: issuer,
        aud: vec![audience],
        iat: now as usize,
        exp: expiration,
        identity,
    };

    let private_key_pem_path =
        std::env::var("STDB_CERT_PATH").expect("Missing STDB_CERT_PATH env var");
    let pem = fs::read_to_string(&private_key_pem_path).await?;
    let kid = compute_kid(&pem).expect("compute kid failed");
    info!("Keyid for spacetimedb token: {:?}", &kid);
    // Build JWT header
    let mut header = Header::new(Algorithm::RS256);
    header.kid = Some("C8ZRZRC1HSfSMq-dSXLGveByrpMwYYORNgkds6Pmn9U".to_string());

    // Create encoding key from PEM
    let encoding_key = EncodingKey::from_rsa_pem(pem.as_bytes())
        .map_err(|e| AppError(anyhow::anyhow!("Failed to parse private key PEM: {}", e)))?;
    
    let token = encode(&header, &claims, &encoding_key)
        .map_err(|e| {
            error!("JWT encoding failed: {:?}", e);
            AppError(anyhow::anyhow!("JWT encoding failed: {}", e))
        })?;

    info!("Generated token with kid");
    Ok(token)
}


fn derive_identity_from_claims(claims: &UserClaims) -> String {
    let input = claims.sub.as_bytes(); // typically the UUID or user ID
    let mut hasher = Sha256::new();
    hasher.update(input);
    let hash = hasher.finalize();
    hex::encode(hash)
}


use std::net::SocketAddr;
use std::sync::Arc;
use anyhow::{anyhow, Context};
use axum::extract::{ConnectInfo, State};
use axum::http::{HeaderMap};
use axum::Json;
use axum::response::IntoResponse;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use sqlx::{query_as};
use uuid::Uuid;
use crate::db::user::User;
use crate::error::AppError;
use crate::routers::user::AppState;
use crate::routers::user::generate_tokens::{generate_access_token, generate_refresh_token};
use crate::shared::{RefreshClaims, TokenResponse};

#[derive(Serialize, Deserialize)]
pub struct RenewRequest {
    refresh_token: String,
}

// dosnt overwrite old
pub async fn renew(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RenewRequest>
) -> Result<impl IntoResponse, AppError> {
    let user_agent = headers
        .get("USER_AGENT")
        .ok_or_else(|| AppError(anyhow!("No user agent")))?
        .to_str()
        .map_err(|_| AppError(anyhow!("Invalid user agent")))?;
    
    let refresh_token = payload.refresh_token;
    let refresh_secret = std::env::var("REFRESH_JWT_SECRET")
        .expect("REFRESH_JWT_SECRET not set");
    let claims = decode::<RefreshClaims>(
        &refresh_token,
        &DecodingKey::from_secret(refresh_secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    let ip_matches = claims.claims.ip == addr.ip().to_string();
    if !ip_matches {
        return Err(AppError(anyhow::anyhow!("IP mismatch on refresh")));
    }
    let user_id = Uuid::parse_str(&claims.claims.sub).expect("Unable to parse user id");
    let user: User = query_as!(
            User,
            r#"
                SELECT id, email, password_hash, is_premium, created_at as "created_at!: chrono::DateTime<chrono::Utc>"
                FROM users
                WHERE id = $1
            "#,
            &user_id
        )
        .fetch_one(state.db_pool.as_ref())
        .await
        .context("User not found")?;

    let access_token = generate_access_token(&user)?;
    let refresh_token = generate_refresh_token(&state, &user.id.to_string(), &addr.ip().to_string(), user_agent).await?;
    

   

    Ok(Json(TokenResponse { access_token, refresh_token }))
}


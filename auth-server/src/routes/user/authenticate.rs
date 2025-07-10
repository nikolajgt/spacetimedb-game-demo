use std::sync::Arc;
use anyhow::Context;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use crate::{AppState};
use crate::db::schemas::User;
use crate::error::AppError;
use crate::shared::{UserClaims, TokenResponse};
use crate::tools::password::verify_password;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}


pub async fn authenticate(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    let user: User = query_as!(
            User,
            r#"
                SELECT id, email, password_hash, is_premium, created_at as "created_at!: chrono::DateTime<chrono::Utc>"
                FROM users
                WHERE email = $1
            "#,
            &payload.email
        )
        .fetch_one(&state.db_pool)
        .await
        .context("User not found")?;
    
    if !verify_password(&payload.password, &user.password_hash) {
        return Err(AppError(anyhow::anyhow!("Wrong password"))); // maybe a better error message
    }

    // continue as normal
    let now = Utc::now().timestamp();
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;
    
    let claims = UserClaims {
        iss: std::env::var("USER_JWT_AUDIENCE")?,
        sub: user.id.to_string(),
        iat: now,
        email: user.email.clone(),
        is_premium: user.is_premium,
        exp: expiration,
    };

    let secret = std::env::var("USER_JWT_SECRET").expect("USER_JWT_SECRET not set");
    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    ).map_err(|e| AppError(anyhow::anyhow!("JWT encoding failed: {}", e)))?;

    Ok(Json(TokenResponse {
        access_token: token,
        refresh_token: "token".to_string(),
    }))
}
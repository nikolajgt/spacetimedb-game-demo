use std::sync::Arc;
use anyhow::Context;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use log::warn;
use crate::{AppState, SECRET_KEY};
use crate::db::schemas::User;
use crate::error::AppError;
use crate::routes::shared::{Claims, TokenResponse};
use crate::tools::validate::{is_secure_password, is_valid_email};

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}


pub async fn authenticate(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, AppError> {
    if !is_valid_email(&payload.email) || !is_secure_password(&payload.password) {
        return Err(AppError(anyhow::anyhow!("Invalid credentials")));
    }

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

    // continue as normal
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(15))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        is_premium: user.is_premium,
        exp: expiration,
    };

    let token = encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY),
    ).map_err(|e| AppError(anyhow::anyhow!("JWT encoding failed: {}", e)))?;

    Ok(Json(TokenResponse {
        access_token: token,
        refresh_token: "token".to_string(),
    }))
}
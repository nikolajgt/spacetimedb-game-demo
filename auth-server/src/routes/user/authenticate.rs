use std::net::SocketAddr;
use std::sync::Arc;
use anyhow::{anyhow, Context};
use axum::extract::{ConnectInfo, State};
use axum::http::{header, HeaderMap};
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use crate::{AppState};
use crate::db::schemas::User;
use crate::error::AppError;
use crate::routes::user::generate_tokens::{generate_access_token, generate_refresh_token};
use crate::shared::{UserClaims, TokenResponse, RefreshClaims};
use crate::tools::password::verify_password;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}


pub async fn authenticate(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
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
        return Err(AppError(anyhow::anyhow!("Wrong password")));
    }

    let user_agent = headers
        .get(header::USER_AGENT)
        .ok_or_else(|| AppError(anyhow!("No user agent")))?
        .to_str()
        .map_err(|_| AppError(anyhow!("Invalid user agent")))?;

    let access_token = generate_access_token(&user)?;
    let refresh_token = generate_refresh_token(&state, &user.id.to_string(), &addr.ip().to_string(), user_agent).await?;

    Ok(Json(TokenResponse {
        access_token,
        refresh_token,
    }))
}



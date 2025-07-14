use axum::http::StatusCode;
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use sqlx::query;
use uuid::Uuid;
use crate::AppState;
use crate::db::schemas::User;
use crate::error::AppError;
use crate::shared::{RefreshClaims, UserClaims};

pub fn generate_access_token(user: &User) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let jwt_exp = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = UserClaims {
        iss: std::env::var("USER_JWT_AUDIENCE")?,
        sub: user.id.to_string(),
        iat: now,
        email: user.email.clone(),
        is_premium: user.is_premium,
        exp: jwt_exp,
    };

    let secret = std::env::var("USER_JWT_SECRET").expect("USER_JWT_SECRET not set");
    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
        .map_err(|e| AppError(anyhow::anyhow!("JWT encoding failed: {}", e)))
}

pub async fn generate_refresh_token(
    app_state: &AppState,
    user_id: &str,
    ip: &str,
    user_agent: &str
) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let refresh_exp = Utc::now()
        .checked_add_signed(Duration::days(7))
        .unwrap()
        .timestamp();

    let refresh_claims = RefreshClaims {
        sub: user_id.to_string(),
        ip: ip.to_string(),
        iat: now,
        exp: refresh_exp as usize,
        user_agent: user_agent.to_string(),
    };

    let refresh_secret = std::env::var("USER_JWT_REFRESH_SECRET")
        .expect("REFRESH_JWT_SECRET not set");

    let refresh_token = encode(
        &Header::new(Algorithm::HS256),
        &refresh_claims,
        &EncodingKey::from_secret(refresh_secret.as_bytes()),
    )
        .map_err(|e| AppError(anyhow::anyhow!("Refresh JWT encoding failed: {}", e)))?;
    let user_id = Uuid::parse_str(&refresh_claims.sub).expect("Unable to parse user id");
    let refresh_exp = chrono::DateTime::<Utc>::from_timestamp(refresh_exp, 0);

    query!(
        r#"
            INSERT INTO refresh_tokens (
                user_id,
                refresh_token,
                ip_address,
                user_agent,
                expires_at
            )
            VALUES ($1, $2, $3, $4, $5)
        "#,
        &user_id,
        &refresh_token,
        &ip,
        user_agent,
        refresh_exp,
        )
        .execute(&app_state.db_pool)
        .await
        .map_err(|err| {
            AppError(anyhow::anyhow!("Insert error: {}", err))
        })?;

    Ok(refresh_token)
}
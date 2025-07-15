use std::sync::Arc;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::{query, query_scalar};
use uuid::Uuid;
use crate::routers::user::AppState;
use crate::tools::password::hash_password;

#[derive(Deserialize)]
pub struct RegisterRequest {
    email: String,
    password: String,
}


pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if !is_valid_email(&payload.email) {
        return Err((StatusCode::BAD_REQUEST, "Invalid email".into()));
    }

    if !is_secure_password(&payload.password) {
        return Err((StatusCode::BAD_REQUEST, "Weak password".into()));
    }

    let existing: bool = query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM users WHERE email = $1
            )
            "#,
            payload.email
        )
        .fetch_one(state.db_pool.as_ref()) // ✅ This is simpler and avoids `conn`
        .await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
        )}).unwrap().unwrap();

    if existing {
        return Err((StatusCode::BAD_REQUEST, "Already registered".into()));
    }

    let id = Uuid::new_v4();
    let password_hash = hash_password(&payload.password);

    query!(
        r#"
            INSERT INTO users (id, email, password_hash)
            VALUES ($1, $2, $3)
        "#,
        &id,
        &payload.email,
        &password_hash
        )
        .execute(state.db_pool.as_ref())
        .await
        .map_err(|err| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Insert error: {}", err),
            )
        })?;


    Ok("Registration completed")
}

pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}


pub fn is_secure_password(password: &str) -> bool {
    password.len() >= 8 && password.chars().any(char::is_uppercase)
}
use std::sync::Arc;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::{query};
use uuid::Uuid;
use crate::error::AppError;
use crate::routers::stdb::StdbAppState;
use crate::tools::header::extract_auth_token;
use crate::tools::validate_tokens::validate_user_token;

#[derive(Deserialize)]
pub struct CharacterCreateRequest {
    pub name: String
} 

pub async fn create_character(
    headers: HeaderMap,
    State(state): State<Arc<StdbAppState>>,
    Json(payload): Json<CharacterCreateRequest>,
) -> Result<impl IntoResponse, AppError> {

    let auth_token = extract_auth_token(&headers)?;
    let user_claims = validate_user_token(&auth_token)
        .map_err(|err| AppError(anyhow::anyhow!("Invalid user token: {:?}", err)))?;

    let user_id = Uuid::parse_str(&user_claims.sub)?;
    query!(
        r#"
            INSERT INTO characters (user_id, name)
            VALUES ($1, $2)
            RETURNING id, user_id, name, level, created_at
        "#,
        user_id,
        payload.name
    )
        .fetch_one(state.db_pool.as_ref())
        .await
        .map_err(|_| AppError(anyhow::anyhow!("Failed to insert character")))?;


    Ok(())
}

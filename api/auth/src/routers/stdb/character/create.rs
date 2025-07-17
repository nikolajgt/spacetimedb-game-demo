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
        .map_err(|err| AppError(anyhow::anyhow!("Invalid auth token: {:?}", err)))?;

    let user_id = Uuid::parse_str(&user_claims.sub)?;
    let character = query!(
            r#"
                INSERT INTO characters (user_id, name)
                VALUES ($1, $2)
                RETURNING id
            "#,
            user_id,
            payload.name
        )
        .fetch_one(state.db_pool.as_ref())
        .await
        .map_err(|_| AppError(anyhow::anyhow!("Failed to insert character")))?;

    sqlx::query!(
        r#"
            INSERT INTO character_movements (
                character_id, pos_x, pos_y, pos_z, dir_x, dir_y, dir_z, mode
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        "#,
        character.id,
        0.0_f32,
        0.0_f32,
        0.0_f32,
        0.0_f32,
        0.0_f32,
        0.0_f32,
        0_i16  
    )
        .execute(state.db_pool.as_ref())
        .await
        .map_err(|_| AppError(anyhow::anyhow!("Failed to insert character movement")))?;
    

    Ok(())
}

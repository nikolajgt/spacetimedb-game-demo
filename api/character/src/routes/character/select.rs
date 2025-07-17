use std::sync::Arc;
use axum::extract::State;
use axum::http::HeaderMap;
use axum::Json;
use axum::response::IntoResponse;
use log::info;
use serde::{Deserialize, Serialize};
use sqlx::query_as;
use uuid::Uuid;
use crate::db::character::Character;
use crate::error::AppError;
use crate::services::AppState;
use crate::tools::header::extract_auth_token;

#[derive(Deserialize)]
pub struct CharacterSelectRequest {
    pub character_id: String
}
#[derive(Serialize)]
pub struct CharacterAuthToken {
    pub token: String,
}

pub async fn select_character(
    headers: HeaderMap,
    State(state): State<Arc<AppState>>,  
    Json(payload): Json<CharacterSelectRequest>, 
) -> Result<impl IntoResponse, AppError> {

    let auth_token = extract_auth_token(&headers)?;
    let user_claims = validate_user_token(&auth_token)
        .map_err(|err| AppError(anyhow::anyhow!("Invalid auth token: {:?}", err)))?;
    
    let character_id = Uuid::parse_str(&payload.character_id)?;
    let user_id = Uuid::parse_str(&user_claims.sub)?;
    // query is only for guarding and can be made better
    let character = query_as!(
            Character,
            r#"
                SELECT id, user_id, name, level, created_at as "created_at!: chrono::DateTime<chrono::Utc>"
                FROM characters
                WHERE id = $1 AND user_id = $2
            "#,
            character_id,
            user_id
        )
        .fetch_one(state.db_pool.as_ref())
        .await
        .map_err(|_| AppError(anyhow::anyhow!("Character not found for auth with id: {}", &payload.character_id)))?;
    
    let (character_token, claims) = state.character_token_handler.generate_character_token(&payload.character_id, &user_claims)
        .map_err(|err| AppError(anyhow::anyhow!("Failed to issue character token: {:?}", err)))?;
    transfer_character(state.db_pool.clone(), "game-demo".to_string(), &character_token, &claims).await?;
    info!("returns");
    
    Ok(Json(CharacterAuthToken { token: character_token }))
}


use std::sync::Arc;
use serde::Deserialize;
use sqlx::{query_as, FromRow, Pool, Postgres};
use uuid::Uuid;
use crate::db::character::{Character, CharacterMovement};
use crate::error::AppError;
use crate::shared::CharacterClaims;

#[derive(Deserialize)]
pub struct IdentityResponse {
    identity: String
}


pub async fn transfer_character(
    db_pool: Arc<Pool<Postgres>>,
    selected_server: String,
    character_token: &String,
    character_claims: &CharacterClaims
) -> Result<(), AppError> {
    let (character, character_movement) = extract_data_postgresql(&db_pool, &character_claims).await?;
    insert_data_stdb(character_token, selected_server, character, character_movement).await?;
    Ok(())
}

async fn extract_data_postgresql(
    db_pool: &Arc<Pool<Postgres>>,
    character_claims: &CharacterClaims
) -> Result<(Character, CharacterMovement), AppError> {
    let character_id = Uuid::parse_str(&character_claims.character_id)?;
    let user_id = Uuid::parse_str(&character_claims.sub)?;

    let response = tokio::try_join!(
          sqlx:: query_as!(
                Character,
                r#"
                    SELECT id, user_id, name, level, created_at as "created_at!: chrono::DateTime<chrono::Utc>"
                    FROM characters
                    WHERE id = $1 AND user_id = $2
                "#,
                &character_id,
                &user_id
            )
            .fetch_one(db_pool.as_ref()),
            sqlx::query_as!(
                CharacterMovement,
                r#"
                    SELECT * FROM character_movements
                    WHERE character_id = $1
                "#,
                &character_id,
            )
            .fetch_one(db_pool.as_ref())
        )?;

    Ok(response)
}

async fn insert_data_stdb(
    character_access_token: &String,
    selected_server: String,
    character: Character,
    character_movement: CharacterMovement,
) -> Result<(), AppError> {
    let identity = reqwest::Client::new()
        .get("http://127.0.0.1:3000/v1/identity")
        .bearer_auth(&character_access_token) // Optional: if DB requires authentication
        .send()
        .await?
        .json::<IdentityResponse>()
        .await?
        .identity;

    let combined_sql = format!(
        "
            INSERT INTO characters (character_id, identity, name, level) VALUES ({}, '{}', '{}', {});
            INSERT INTO character_movement (character_id, identity, pos_x, pos_y, pos_z, dir_x, dir_y, dir_z, mode)
                VALUES ({}, '{}', {}, {}, {}, {}, {}, {}, {});
            ",
        &character.id, identity, &character.name, &character.level,
        &character.id, identity, &character_movement.pos_x, &character_movement.pos_y, &character_movement.pos_z, &character_movement.dir_x, &character_movement.dir_y, &character_movement.dir_z, &character_movement.mode,
    );

    let response = reqwest::Client::new()
        .post(&format!("http://127.0.0.1:3000/v1/database/{}/sql", selected_server))
        .header("Content-Type", "text/plain")
        .bearer_auth(&character_access_token) // Optional: if DB requires authentication
        .body(combined_sql)
        .send()
        .await?;

    Ok(())
}



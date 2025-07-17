use std::sync::Arc;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::services::token_handler::TokenHandler;

mod transfer_handler;
mod token_handler;
mod user_auth_handler;

#[derive(Clone)]
pub struct AppState {
    pub character_token_handler: TokenHandler,
    pub db_pool: Arc<Pool<Postgres>>,
}


impl AppState {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/game-demo".to_string());

        let db_pool: Pool<Postgres> = PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url)
            .await
            .expect("Failed to connect to Postgres");

        let pool = Arc::new(db_pool);
        
        Self {
            character_token_handler: TokenHandler::new(),
            db_pool: pool
        }
    }
}
mod user;
mod tools;
mod db;
mod routes;
mod error;
mod shared;

use std::sync::Arc;
use axum::{routing::post, Router};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::tools::jwk_builder::{JwkBuilder, JwkSet};

#[derive(Clone)]
pub struct AppState {
    db_pool: Pool<Postgres>,
    set_keys: JwkSet,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/game-demo".to_string());
    
    
    let db_pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let cert_path = std::env::var("STDB_CERT_PATH").expect("Missing STDB_CERT_PATH env var");
    let set_keys = JwkBuilder::from_pem_file(cert_path).await?;
    let app_state = Arc::new(AppState {
        db_pool,
        set_keys
    });
    
    let app = Router::new()
        .merge(routes::user_router(app_state.clone()))
        .merge(routes::stdb_router(app_state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.expect("Failed to create JWK");
    axum::serve(listener, app).await.expect("Failed to start axum server");
    Ok(())
}












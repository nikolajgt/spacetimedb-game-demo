mod user;
mod tools;
mod db;
mod routers;
mod error;
mod shared;

use std::net::SocketAddr;
use std::sync::Arc;
use axum::Router;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;




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

    let db_pool = Arc::new(db_pool);
    
    let app = Router::new()
        .merge(routers::user::user_router(db_pool.clone()))
        .merge(routers::stdb::stdb_router(db_pool.clone()))
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.expect("Failed to create JWK");

    axum::serve(listener, app).await.expect("Failed to start axum server");
    Ok(())
}












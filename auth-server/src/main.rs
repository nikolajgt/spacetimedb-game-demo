mod user;
mod tools;
mod db;
mod routes;
mod error;

use std::sync::Arc;
use axum::{routing::post, Router};
use axum::{response::IntoResponse};
use serde::{Deserialize, Serialize};
use argon2::{PasswordHasher, PasswordVerifier, };
use sqlx::{Acquire, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::routes::authenticate::authenticate;
use crate::routes::register_user::register;
use crate::routes::renew_tokens::renew;

const SECRET_KEY: &[u8] = b"super_secret_key_1234567890";




#[derive(Clone)]
pub struct AppState {
    db_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    
    // build database connection pool
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/game-demo".to_string());

    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    let app_state = Arc::new(AppState {
        db_pool: pool,
    });
    // build our application with a route
    let app = Router::new()
        .route("/register", post(register))
        .route("/authenticate", post(authenticate))
        .route("/renew", post(renew))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}












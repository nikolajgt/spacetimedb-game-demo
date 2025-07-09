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
use crate::routes::authenticate::authenticate;
use crate::routes::identity::identity;
use crate::routes::register_user::register;
use crate::routes::renew_tokens::renew;





#[derive(Clone)]
pub struct AppState {
    db_pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
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
        .route("/identity", post(identity))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}












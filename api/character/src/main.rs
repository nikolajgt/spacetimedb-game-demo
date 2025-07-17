mod routes;
mod error;
mod services;
mod tools;
mod db;

use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use axum::http::{header, HeaderValue, Method};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use crate::services::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let state  = Arc::new(AppState::new().await);
    
    let app = Router::new()
        .merge(routes::endpoints())
        .layer(cors)
        .with_state(state)
        .into_make_service_with_connect_info::<SocketAddr>();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3010").await.expect("Failed to create JWK");

    axum::serve(listener, app).await.expect("Failed to start axum character-game-server");
    Ok(())
}



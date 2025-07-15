use std::sync::Arc;
use axum::http::{header, HeaderValue, Method};
use axum::Router;
use axum::routing::{get, post};
use sqlx::{Pool, Postgres};
use tower_http::cors::CorsLayer;
use crate::routers::stdb::character::create::create_character;
use crate::routers::stdb::character::select::select_character;
use crate::routers::stdb::character_token_handler::CharacterTokenHandler;
use crate::routers::stdb::jwks::{jwks, openid_config};

pub mod jwks;
pub mod character;
pub mod character_token_handler;


pub fn stdb_router(
    pool: Arc<Pool<Postgres>>,
) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let app_state = Arc::new(StdbAppState {
        character_token_handler: CharacterTokenHandler::new(),
        db_pool: pool
    });

    Router::new()
        .route("/.well-known/openid-configuration", get(openid_config))
        .route("/.well-known/jwks.json", get(jwks))
        .route("/api/character/create", post(create_character))
        .route("/api/character/select", post(select_character))
        .layer(cors)
        .with_state(app_state)
}

#[derive(Clone)]
pub struct StdbAppState  {
    pub character_token_handler: CharacterTokenHandler,
    pub db_pool: Arc<Pool<Postgres>>,
}





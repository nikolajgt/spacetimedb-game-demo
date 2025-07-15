use std::sync::Arc;
use axum::http::{header, Method};
use axum::Router;
use axum::routing::post;
use sqlx::{Pool, Postgres};
use tower_http::cors::{Any, CorsLayer};
use crate::routers::user::authenticate::authenticate;
use crate::routers::user::register_user::register;
use crate::routers::user::renew_tokens::renew;

pub mod authenticate;
pub mod register_user;
pub mod renew_tokens;
mod generate_tokens;


#[derive(Clone)]
pub struct AppState {
    db_pool: Arc<Pool<Postgres>>,
}

pub fn user_router(
    pool: Arc<Pool<Postgres>>,
) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any) // allow any origin
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);
    let app_state = Arc::new(AppState {
        db_pool: pool
    });
    
    Router::new()
        .route("/api/register", post(register))
        .route("/api/authenticate", post(authenticate))
        .route("/api/renew", post(renew))
        .layer(cors)
        .with_state(app_state)
}
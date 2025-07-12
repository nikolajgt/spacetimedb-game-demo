mod user;
mod stdb;

use std::sync::Arc;
use axum::http::{header, HeaderValue, Method};
use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{Any, CorsLayer};
use crate::AppState;
use crate::routes::stdb::identity::identity;
use crate::routes::stdb::jwks::{jwks, openid_config};
use crate::routes::user::authenticate::authenticate;
use crate::routes::user::register_user::register;
use crate::routes::user::renew_tokens::renew;

// user: allow all origins
pub fn user_router(app_state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any) // allow any origin
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    Router::new()
        .route("/api/register", post(register))
        .route("/api/authenticate", post(authenticate))
        .route("/api/renew", post(renew))
        .layer(cors)
        .with_state(app_state)
}

// stdb: allow only localhost:3000
pub fn stdb_router(app_state: Arc<AppState>
) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);
    
    Router::new()
        .route("/identity", post(identity))
        .route("/.well-known/openid-configuration", get(openid_config))
        .route("/.well-known/jwks.json", get(jwks))
        .layer(cors)
        .with_state(app_state)
}
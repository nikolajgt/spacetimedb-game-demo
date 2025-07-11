mod user;
mod stdb;

use std::sync::Arc;
use axum::http::{header, HeaderValue, Method};
use axum::Router;
use axum::routing::{get, post};
use tower_http::cors::{Any, CorsLayer};
use crate::AppState;
use crate::routes::stdb::identity::identity;
use crate::routes::stdb::jwks::jwks;
use crate::routes::user::authenticate::authenticate;
use crate::routes::user::register_user::register;
use crate::routes::user::renew_tokens::renew;
use crate::tools::jwk_builder::JwkSet;

// user: allow all origins
pub fn user_router(app_state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any) // allow any origin
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    Router::new()
        .route("/register", post(register))
        .route("/authenticate", post(authenticate))
        .route("/renew", post(renew))
        .layer(cors)
        .with_state(app_state)
}

// stdb: allow only localhost
pub fn stdb_router(app_state: Arc<AppState>
) -> Router {
    // let cors = CorsLayer::new()
    //     .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    //     .allow_methods([Method::GET, Method::POST])
    //     .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    let cors = CorsLayer::new()
        .allow_origin(Any) // allow any origin
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);
    
    Router::new()
        .route("/identity", post(identity))
        .route("/.well-known/jwks.json", get(jwks))
        .layer(cors)
        .with_state(app_state)
}
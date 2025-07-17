mod character;
mod jwks;

use std::sync::Arc;
use axum::Router;
use axum::routing::{get, post};
use crate::routes::character::create::create_character;
use crate::routes::character::select::select_character;
use crate::routes::jwks::{jwks, openid_config};
use crate::services::AppState;

pub fn endpoints() -> Router<Arc<AppState>> {
    let router = Router::new()
        .route("/.well-known/openid-configuration", get(openid_config))
        .route("/.well-known/jwks.json", get(jwks))
        .route("/api/character/create", post(create_character))
        .route("/api/character/select", post(select_character));
    
    router
}
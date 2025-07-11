use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use log::info;
use serde::{Deserialize, Serialize};
use crate::AppState;

pub async fn jwks(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    info!("SPACETIME CALLED FOR JWKS");
    Json(state.set_keys.clone())
}

#[derive(Serialize, Deserialize)]
struct OpenIdConfig {
    issuer: String,
    jwks_uri: String,
}
pub async fn openid_config() -> impl IntoResponse {
    info!("JWKS OPENID CONFIG FOR JWKS");
    Json(OpenIdConfig {
        issuer: "http://localhost:3010".to_string(),
        jwks_uri: "http://localhost:3010/.well-known/jwks.json".to_string(),
    })

}
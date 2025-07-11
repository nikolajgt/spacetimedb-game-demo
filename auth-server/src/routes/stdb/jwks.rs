use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use log::info;
use crate::AppState;

pub async fn jwks(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    info!("SPACETIME CALLED FOR JWKS");
    Json(state.set_keys.clone())
}


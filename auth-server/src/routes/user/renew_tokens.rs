use std::sync::Arc;
use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use crate::AppState;
use crate::routes::user::authenticate::LoginRequest;
use crate::shared::TokenResponse;

pub async fn renew(
    State(pool): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>
) -> impl IntoResponse  {

    Json(TokenResponse { access_token: "token".to_string(), refresh_token: "refresh_token".to_string() })
}


use axum::http::{HeaderMap};
use axum::response::IntoResponse;
use log::{info};
use crate::error::AppError;
use crate::tools::header::extract_auth_token;

pub async fn delete_character(
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    let auth_token = extract_auth_token(&headers)?;

    // let claims = match validate_user_token(auth_token) {
    //     Ok(claims) => claims,
    //     Err(err) => {
    //         error!("{}", err);
    //         return Err(AppError(anyhow!(err)));
    //     }
    // };
    // let token = issue_stdb_token(&claims).await?;

    info!("Returning identity game token");
    Ok(())
}

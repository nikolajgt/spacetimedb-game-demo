use anyhow::anyhow;
use axum::http::{header, HeaderMap};
use crate::error::AppError;

pub fn extract_auth_token(headers: &HeaderMap) -> Result<String, AppError> {
    let auth_token = headers
        .get(header::AUTHORIZATION)
        .ok_or_else(|| AppError(anyhow!("No authorization token attached")))?
        .to_str()?
        .strip_prefix("Bearer ")
        .ok_or_else(|| AppError(anyhow!("No Bearer ")))?;
    
    Ok(auth_token.to_string())
}

pub fn extract_user_agent(headers: &HeaderMap) -> Result<String, AppError> {
    let auth_token = headers
        .get(header::USER_AGENT)
        .ok_or_else(|| AppError(anyhow!("No user-agent header attached")))?
        .to_str()
        .map_err(|_| AppError(anyhow!("No user-agent header ")))?;

    Ok(auth_token.to_string())
}

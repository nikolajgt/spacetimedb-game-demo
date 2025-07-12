use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    pub iss: String,
    pub sub: String,
    pub iat: i64,
    pub email: String,
    pub is_premium: bool,
    pub exp: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub ip: String,
    pub exp: usize,
    pub iat: i64,
}


#[derive(Serialize, Deserialize)]
pub struct SpacetimeClaims {
    pub sub: String,
    pub iss: String,
    pub aud: Vec<String>,
    pub iat: usize,
    pub exp: usize,
  //  pub hex_identity: String,
}
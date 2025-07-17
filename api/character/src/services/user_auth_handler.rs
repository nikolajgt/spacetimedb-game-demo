use crate::error::AppError;

// for now auth uses secret, later it should use pem with public keys, so we can fetch public instead
// of calling auth to validate it
pub struct UserAuthHandler {
    pub base_url: String,
}

impl UserAuthHandler {
    pub fn new() -> Self {
        let path = std:: env::var("AUTH_API_URL").expect("Missing STDB_CERT_PATH env var");
        Self {
            base_url: path
        }
    }
    
    
    pub async fn validate_user_token(&self, access_token: &String) -> Result<bool, AppError> {
        let base_url = &self.base_url;
        let endpoint: format!("{}/api/validate", base_url);
        match reqwest::Client::new()
            .get(&endpoint)
            .bearer_auth(access_token)
            .send()
            .await? {
            Ok(_) => Ok(true),
            Err(err) => Err(AppError::from(err)),
        }
    }

    async fn fetch_public_keys(&self, access_token: &String) -> Result<bool, AppError> {
        let endpoint: format!("{}/api/validate", base_url);
        match reqwest::Client::new()
            .get(&endpoint)
            .bearer_auth(access_token)
            .send()
            .await? {
            Ok(_) => Ok(true),
            Err(err) => Err(AppError::from(err)),
        }
    }
}
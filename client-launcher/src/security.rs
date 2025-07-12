use keyring::Entry;
use serde::{Deserialize, Serialize};
use crate::Screen;

const SERVICE: &str = "ratatui-launcher";

pub struct AuthenticationState {
    refresh_token: Option<String>,
    access_token: Option<String>,
    auth_server_addr: String
}

#[derive(Serialize, Deserialize)]
struct TokenSet {
    access_token: String,
    refresh_token: String,
}


impl AuthenticationState {
    
    pub fn default() -> Self
    {
        let auth_server_addr = std::env::var("AUTH_SERVER_ADDR")
            .expect("Environment variable AUTH_SERVER_ADDR is required.");
        Self {
            access_token: None,
            refresh_token: None,
            auth_server_addr
        }
    }
    pub async fn attempt_auto_login(&mut self) -> Result<bool, String> {
        let refresh_token = match get_local_refresh() {
            Ok(t) => t,
            Err(_) => return Ok(false),
        };

        match self.fetch_new_tokens(&refresh_token).await {
            Ok(new_tokens) => {
                if let Err(e) = store_local_refresh(&new_tokens.refresh_token) {
                    eprintln!("Failed to update stored refresh token: {e}");
                }
                
                self.access_token = Some(new_tokens.access_token);
                self.refresh_token = Some(new_tokens.refresh_token);

                Ok(true)
            }
            Err(e) => {
                eprintln!("Token validation failed: {e}");
                Ok(false)
            }
        }
    }

    pub async fn validate_token(&self, access_token: &String) -> Result<(), String> {
        let client = reqwest::Client::new();
        let url = format!("{}/validate", self.auth_server_addr.trim_end_matches('/'));

        match client.get(&url)
            .bearer_auth(access_token)
            .send()
            .await
        {
            Ok(response) if response.status().is_success() => Ok(()),
            Ok(response) => Err(format!("Token validation failed: HTTP {}", response.status())),
            Err(e) => Err(format!("Request error while validating token: {e}")),
        }
    }

    pub async fn fetch_new_tokens(&self, refresh_token: &String) -> Result<TokenSet, String> {
        let client = reqwest::Client::new();
        let url = format!("{}/refresh", self.auth_server_addr.trim_end_matches('/'));

        let res = client
            .post(&url)
            .json(&serde_json::json!({ "refresh_token": refresh_token }))
            .send()
            .await
            .map_err(|e| format!("Failed to send refresh request: {e}"))?;

        if res.status().is_success() {
            res.json::<TokenSet>()
                .await
                .map_err(|e| format!("Failed to deserialize refresh response: {e}"))
        } else {
            Err(format!("Refresh token failed with status: {}", res.status()))
        }
    }
}
fn store_local_refresh(refresh_token: &String) -> Result<(), Box<dyn std::error::Error>> {
    Entry::new(SERVICE, "auth")?.set_password(&refresh_token)?;
    Ok(())
}

fn get_local_refresh() -> Result<String, Box<dyn std::error::Error>> {
    let token = Entry::new(SERVICE, "auth")?.get_password()?;
    Ok(token)
}

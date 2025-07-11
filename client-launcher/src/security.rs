use keyring::Entry;
use serde::{Deserialize, Serialize};
use crate::Screen;

#[derive(Serialize, Deserialize)]
struct StoredTokens {
    access_token: String,
    refresh_token: String,
}

const SERVICE: &str = "ratatui-launcher";

pub async fn attempt_auto_login() -> Screen {
    match get_tokens() {
        Ok(tokens) if validate_token(&tokens.access_token).await => Screen::Home,
        Ok(tokens) => {
            match refresh_token(&tokens.refresh_token).await {
                Some(new_tokens) => {
                    store_tokens(&new_tokens).ok();
                    Screen::Home
                }
                None => {
                   // delete_tokens().ok();
                    Screen::Login
                }
            }
        }
        Err(_) => Screen::Login,
    }
}

async fn validate_token(token: &str) -> bool {
    let client = reqwest::Client::new();
    let res = client
        .get("http://localhost:3010/api/auth/validate")
        .bearer_auth(token)
        .send()
        .await;

    matches!(res, Ok(r) if r.status().is_success())
}

async fn refresh_token(refresh: &str) -> Option<StoredTokens> {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:3010/api/auth/refresh")
        .json(&serde_json::json!({ "refresh_token": refresh }))
        .send()
        .await
        .ok()?;

    res.json::<StoredTokens>().await.ok()
}


fn store_tokens(tokens: &StoredTokens) -> Result<(), Box<dyn std::error::Error>> {
    let serialized = serde_json::to_string(tokens)?;
    Entry::new(SERVICE, "auth")?.set_password(&serialized)?;
    Ok(())
}

fn get_tokens() -> Result<StoredTokens, Box<dyn std::error::Error>> {
    let val = Entry::new(SERVICE, "auth")?.get_password()?;
    Ok(serde_json::from_str(&val)?)
}


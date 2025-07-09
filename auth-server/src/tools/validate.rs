use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use jsonwebtoken::errors::ErrorKind;
use crate::shared::{SpacetimeClaims, UserClaims};

pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.')
}


pub fn is_secure_password(password: &str) -> bool {
    password.len() >= 8 && password.chars().any(char::is_uppercase)
}

pub fn validate_user_token(token: &str) -> Result<UserClaims, String> {
    let secret = std::env::var("AUTH_SECRET").expect("AUTH_SECRET not set");

    // Optional: validate iss, aud, etc.
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.set_required_spec_claims(&["iss", "sub", "exp"]);

    match decode::<UserClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ) {
        Ok(TokenData { claims, .. }) => Ok(claims),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => Err("Token expired".to_string()),
            _ => Err(format!("Invalid token: {:?}", err)),
        },
    }
}

pub fn validate_spacetime_token(token: &str) -> Result<SpacetimeClaims, String> {
    let secret = std::env::var("AUTH_SECRET").expect("AUTH_SECRET not set");

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = true;
    validation.set_required_spec_claims(&["iss", "sub", "exp"]);

    match decode::<SpacetimeClaims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
    ) {
        Ok(TokenData { claims, .. }) => Ok(claims),
        Err(err) => match *err.kind() {
            ErrorKind::ExpiredSignature => Err("Token expired".to_string()),
            _ => Err(format!("Invalid token: {:?}", err)),
        },
    }
}
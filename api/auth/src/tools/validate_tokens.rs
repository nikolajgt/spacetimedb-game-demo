use jsonwebtoken::{decode, Algorithm, DecodingKey, TokenData, Validation};
use jsonwebtoken::errors::ErrorKind;
use crate::error::AppError;
use crate::shared::{CharacterClaims, UserClaims};



pub fn validate_user_token(token: &str) -> Result<UserClaims, AppError> {
    let secret = std::env::var("USER_JWT_SECRET").expect("USER_JWT_SECRET not set");

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
            ErrorKind::ExpiredSignature => Err(AppError(anyhow::anyhow!("Token is expired"))),
            _ =>Err(AppError(anyhow::anyhow!("Unable to decode token: {:?}", err))),
        },
    }
}


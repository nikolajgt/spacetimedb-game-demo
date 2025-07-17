use std::{env, fs};
use anyhow::Context;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rsa::{ RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding};
use rsa::pkcs8::DecodePrivateKey;
use rsa::signature::digest::Digest;
use rsa::traits::PublicKeyParts;
use serde::Serialize;
use crate::error::AppError;
use crate::shared::{CharacterClaims, UserClaims};



#[derive(Serialize, Clone)]
pub struct JwkKey {
    pub kty: &'static str,
    pub alg: &'static str,
    #[serde(rename = "use")]
    pub use_: &'static str,
    pub kid: String,
    pub n: String,
    pub e: String,
}

#[derive(Serialize, Clone)]
pub struct JwkSet {
    pub keys: Vec<JwkKey>,
}


#[derive(Clone)]
pub struct CharacterTokenHandler {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
    kid: String,
    pub jwk: JwkSet,
    issuer: String,
    audience: String,
}

impl CharacterTokenHandler {
    pub fn new() -> Self {
        let pem_path =
            env::var("STDB_CERT_PATH").expect("Missing STDB_CERT_PATH env var");
        let pem = fs::read_to_string(pem_path)
            .expect("Failed to read the private key PEM file");
        let audience = std::env::var("STDB_JWT_AUDIENCE")
            .expect("Missing STDB_JWT_AUDIENCE env var");
        let issuer = std::env::var("STDB_JWT_ISSUER")
            .expect("Missing STDB_JWT_ISSUER env var");

        let private_key = RsaPrivateKey::from_pkcs8_pem(&pem)
            .expect("Failed to parse the private key from PEM");
        let public_key = RsaPublicKey::from(&private_key);
        let kid = compute_kid(&public_key)
            .expect("Failed to compute kid");
        let jwk = to_jwk(&public_key, kid.clone())
            .expect("Failed to generate JWK");
        Self {
            private_key,
            public_key,
            kid,
            jwk,
            issuer,
            audience,
        }
    }

    // Function to generate JWT using the private key
    pub fn generate_character_token(
        &self, 
        character_id: &String, 
        user_claims: &UserClaims
    ) -> Result<(String, CharacterClaims), Box<dyn std::error::Error>> {
        let now = Utc::now();
        let expiration = now
            .checked_add_signed(Duration::hours(24))
            .ok_or_else(|| "Failed to calculate expiration time")?
            .timestamp() as usize;
        
        let character_claims = CharacterClaims {
            sub: user_claims.sub.to_string(),
            iss: self.issuer.clone(),
            aud: vec![self.audience.clone()],
            iat: now.timestamp() as usize,
            exp: expiration,
            character_id: character_id.clone(),
            is_premium: user_claims.is_premium.clone(),
        };
        
        let encoding_key = EncodingKey::from_rsa_pem(self.private_key.to_pkcs1_pem(LineEnding::LF)?.as_bytes())
            .context("Failed to create encoding key from private RSA key")?;
        
        let header = Header::new(Algorithm::RS256);
        let token = encode(&header, &character_claims, &encoding_key)
            .context("Failed to encode JWT")?;

        Ok((token, character_claims))
    }

    /// does not validate token, only extracting claims
    pub fn extract_claims(token: &str) -> Result<CharacterClaims, AppError> {
        let token_data = decode::<CharacterClaims>(
            token,
            &DecodingKey::from_secret("".as_ref()), 
            &Validation::new(Algorithm::RS256),     
        ).map_err(|e| AppError(anyhow::anyhow!("Token decode failed: {}", e)))?;

        Ok(token_data.claims)
    }


    pub fn validate_token(&self, token: &str) -> Result<TokenData<CharacterClaims>, Box<dyn std::error::Error>> {
        let decoding_key = DecodingKey::from_rsa_pem(self.public_key.to_pkcs1_pem(LineEnding::LF)?.as_bytes())
            .context("Failed to create decoding key from public RSA key")?;

        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_issuer(&[&self.issuer]);
        validation.set_audience(&[&self.audience]);

        let token_data = decode::<CharacterClaims>(token, &decoding_key, &validation)
            .context("Failed to decode and validate JWT")?;

        Ok(token_data)
    }
}


pub fn to_jwk(
    public_key: &RsaPublicKey,
    kid: String
) -> Result<JwkSet, Box<dyn std::error::Error>> {
    let n = URL_SAFE_NO_PAD.encode(public_key.n().to_bytes_be());
    let e = URL_SAFE_NO_PAD.encode(public_key.e().to_bytes_be());

    Ok(JwkSet {
        keys: vec![JwkKey {
            kty: "RSA",
            alg: "RS256",
            use_: "sig",
            kid,
            n,
            e,
        }],
    })
}

// Function to compute the Key ID (kid) based on the public key
fn compute_kid(public_key: &RsaPublicKey) -> Result<String, Box<dyn std::error::Error>> {
    let der = public_key.to_pkcs1_der()?;
    let der = der.as_bytes().to_vec();
    let hash = sha2::Sha256::digest(&der);
    let kid = URL_SAFE_NO_PAD.encode(&hash);

    Ok(kid)
}


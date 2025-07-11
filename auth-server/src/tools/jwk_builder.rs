use std::path::Path;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use jsonwebtoken::EncodingKey;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePublicKey};
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1::{DecodeRsaPrivateKey, EncodeRsaPrivateKey};
use rsa::signature::digest::Digest;
use rsa::traits::PublicKeyParts;
use serde::Serialize;
use sha2::Sha256;
use tokio::fs;

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

pub struct JwkBuilder;
impl JwkBuilder {
    pub async fn from_pem_file<P: AsRef<Path>>(path: P) -> Result<JwkSet, Box<dyn std::error::Error>> {
        let pem = fs::read_to_string(path).await?;
        let public_key = RsaPublicKey::from_public_key_pem(&pem)?;

        let private_key = RsaPrivateKey::from_pkcs1_pem(&pem)?; 
        let der = private_key.to_pkcs1_der()?;
        let det_bytes = der.as_bytes();
        let kid = compute_kid_from_der(&det_bytes);

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
}

pub fn compute_kid_from_der(der: &[u8]) -> String {
    let hash = Sha256::digest(der);
    URL_SAFE_NO_PAD.encode(&hash)
}

pub fn compute_kid(pem: &str) -> Result<String, Box<dyn std::error::Error>> {
    let rsa = Rsa::public_key_from_pem(&pem.as_bytes())?;
    let pkey = PKey::from_rsa(rsa)?;
    let der = pkey.public_key_to_der()?; 
    let hash = Sha256::digest(&der);
    Ok(URL_SAFE_NO_PAD.encode(&hash))
}
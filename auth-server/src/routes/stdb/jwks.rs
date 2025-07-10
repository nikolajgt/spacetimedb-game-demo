use axum::Json;
use axum::response::IntoResponse;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rsa::pkcs8::DecodePublicKey;
use rsa::RsaPublicKey;
use rsa::signature::digest::Digest;
use rsa::traits::PublicKeyParts;
use serde::Serialize;
use sha2::Sha256;

#[derive(Serialize)]
struct JwkKey {
    kty: String,
    alg: String,
    #[serde(rename = "use")]
    use_: String,
    kid: String,
    n: String,
    e: String,
}

#[derive(Serialize)]
pub struct JwkSet {
    keys: Vec<JwkKey>,
}



pub async fn jwks() -> impl IntoResponse {
    let public_key_pem = include_str!("../../../../cert/public_key.pem");
    let kid = compute_kid(public_key_pem);
    let jwk_set = generate_jwk(public_key_pem, &kid);
    Json(jwk_set)
}

pub fn generate_jwk(public_key_pem: &str, kid: &str) -> JwkSet {
    let public_key = RsaPublicKey::from_public_key_pem(public_key_pem).unwrap();
    let n_bytes = public_key.n().to_bytes_be();
    let e_bytes = public_key.e().to_bytes_be();
    JwkSet {
        keys: vec![JwkKey {
            kty: "RSA".to_string(),
            alg: "RS256".to_string(),
            use_: "sig".to_string(),
            kid: kid.to_string(),
            n: URL_SAFE_NO_PAD.encode(n_bytes),
            e: URL_SAFE_NO_PAD.encode(e_bytes),
        }],
    }
}

pub fn compute_kid(pem: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(pem.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(hash)
}

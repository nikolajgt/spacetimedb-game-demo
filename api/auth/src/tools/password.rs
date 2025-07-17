use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use once_cell::sync::Lazy;

pub static ARGON2: Lazy<Argon2<'static>> = Lazy::new(|| {
    Argon2::new(
        argon2::Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
});

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(hash).unwrap();
    ARGON2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

pub fn hash_password(password: &str
) -> String {
    let salt = SaltString::generate(&mut OsRng);
    ARGON2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}


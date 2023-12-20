use data_encoding::HEXLOWER;
use once_cell::sync::Lazy;
use rand::Rng;
use ring::{digest, pbkdf2};
use std::{num::NonZeroU32, sync::Arc};

const CREDENTIAL_LEN: usize = digest::SHA256_OUTPUT_LEN;
static PBKDF2_ALG: pbkdf2::Algorithm = pbkdf2::PBKDF2_HMAC_SHA256;
static PBKDF2_ITERATIONS: Lazy<Arc<NonZeroU32>> = Lazy::new(|| {
    Arc::new(NonZeroU32::new(100_000).unwrap()) // handle iterations with user config
});

pub fn hash_password(secret: &str, salt: &str) -> anyhow::Result<String, String> {
    let mut hash = [0u8; CREDENTIAL_LEN];
    let iterations = PBKDF2_ITERATIONS.clone();
    pbkdf2::derive(
        PBKDF2_ALG,
        *iterations,
        salt.as_bytes(),
        secret.as_bytes(),
        &mut hash,
    );

    let password = HEXLOWER.encode(&hash);

    Ok(password)
}

pub fn verify_password(secret: &str, password: &str, salt: &str) -> anyhow::Result<bool> {
    let mut password_vec: Vec<u8> = Vec::new();

    if let Ok(password_bytes) = HEXLOWER.decode(password.as_bytes()) {
        password_vec = password_bytes;
    }

    let iterations = PBKDF2_ITERATIONS.clone();
    match pbkdf2::verify(
        PBKDF2_ALG,
        *iterations,
        salt.as_bytes(),
        secret.as_bytes(),
        &password_vec,
    ) {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

pub fn generate_salt() -> String {
    let mut salt = [0u8; CREDENTIAL_LEN];
    rand::thread_rng().fill(&mut salt[..]);
    HEXLOWER.encode(&salt)
}

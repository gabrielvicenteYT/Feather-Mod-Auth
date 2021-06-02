use crate::utils::error::WebsiteError;
use crate::utils::error::WebsiteError::CryptoError;
use sodiumoxide::crypto::pwhash::argon2id13;

pub fn hash_password(password: String) -> Result<Vec<u8>, WebsiteError> {
    sodiumoxide::init().unwrap();
    match argon2id13::pwhash(
        password.as_bytes(),
        argon2id13::OPSLIMIT_INTERACTIVE,
        argon2id13::MEMLIMIT_INTERACTIVE,
    ) {
        Ok(hash) => Ok(hash.0.to_vec()),
        Err(_) => Err(CryptoError()),
    }
}
pub fn verify_password(password: String, hash: Vec<u8>) -> Result<bool, WebsiteError> {
    sodiumoxide::init().unwrap();
    match argon2id13::HashedPassword::from_slice(&hash) {
        Some(hp) => Ok(argon2id13::pwhash_verify(&hp, password.as_bytes())),
        _ => Err(CryptoError()),
    }
}

use crate::{encoding, types};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use base64::{engine::general_purpose::STANDARD, Engine};
use rand::{rngs::OsRng, Rng};
use sha2::{Digest, Sha256};

pub fn encrypt_password(
    master_password: &str,
    password: &str,
    platform: &str,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut hasher = Sha256::new();
    hasher.update(master_password.as_bytes());
    let hashed_master_password = hasher.finalize();
    let key = Key::<Aes256Gcm>::from_slice(&hashed_master_password);

    let cipher = Aes256Gcm::new(key);

    let nonce_array: [u8; 12] = OsRng.gen();
    let nonce = Nonce::from_slice(&nonce_array);

    let encrypted_password = cipher
        .encrypt(nonce, password.as_bytes())
        .map_err(|e| format!("Encryption failed: {}", e))?;

    let encoded_entry = encoding::encode_entry(platform, &encrypted_password, &nonce_array)?;

    Ok(encoded_entry)
}

pub fn decrypt_password(
    master_password: &str,
    entry: types::Entry,
) -> Result<String, Box<dyn std::error::Error>> {
    let encrypted_password = STANDARD.decode(&entry.password_enc)?;
    let nonce_bytes = STANDARD.decode(&entry.nonce_enc)?;

    if nonce_bytes.len() != 12 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid nonce length.",
        )));
    }
    let nonce = Nonce::from_slice(&nonce_bytes);

    let mut hasher = Sha256::new();
    hasher.update(master_password.as_bytes());
    let hashed_master_password = hasher.finalize();
    let key = Key::<Aes256Gcm>::from_slice(&hashed_master_password);

    let cipher = Aes256Gcm::new(key);

    let decrypted_password = cipher
        .decrypt(nonce, encrypted_password.as_ref())
        .map_err(|e| format!("Decryption failed: {}", e))?;

    let password = String::from_utf8(decrypted_password)?;
    Ok(password)
}

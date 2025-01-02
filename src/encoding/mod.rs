use crate::types;
use base64::{engine::general_purpose::STANDARD, Engine};
use bincode;

pub fn encode_entry(
    platform: &str,
    password: &[u8],
    nonce: &[u8],
) -> Result<Vec<u8>, Box<bincode::ErrorKind>> {
    let new_entry = types::Entry {
        platform: platform.to_string(),
        password_enc: STANDARD.encode(password),
        nonce_enc: STANDARD.encode(nonce),
    };

    let serialized_entry = bincode::serialize(&new_entry)?;

    Ok(serialized_entry)
}

pub fn decode_entry(encoded_entry: &[u8]) -> Result<types::Entry, Box<bincode::ErrorKind>> {
    let decoded_entry: types::Entry = bincode::deserialize(&encoded_entry)?;

    Ok(decoded_entry)
}

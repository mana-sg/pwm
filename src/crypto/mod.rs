use aes_gcm::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng},
    Aes256Gcm,
};

pub fn test() -> Result<(), aes_gcm::Error> {
    let key = Aes256Gcm::generate_key(&mut OsRng);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per message

    let mut buffer: Vec<u8> = Vec::new(); // Note: buffer needs 16-bytes overhead for auth tag
    buffer.extend_from_slice(b"plaintext message");

    // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
    cipher.encrypt_in_place(&nonce, b"", &mut buffer)?;

    // `buffer` now contains the message ciphertext
    assert_ne!(&buffer, b"plaintext message");

    // Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
    cipher.decrypt_in_place(&nonce, b"", &mut buffer)?;
    assert_eq!(&buffer, b"plaintext message");

    Ok(())
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
    pub platform: String,
    pub password_enc: String,
    pub nonce_enc: String,
}

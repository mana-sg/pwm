use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Result, Write};
use std::path::Path;

use base64::{engine::general_purpose::STANDARD, Engine};

use crate::crypto::{decrypt_password, encrypt_password};
use crate::encoding::decode_entry;

pub fn load_stored_password_platforms(file_path: &str) -> Result<Vec<String>> {
    if !Path::new(file_path).exists() {
        File::create(file_path)?;
    }

    let file = OpenOptions::new().read(true).open(file_path)?;

    let reader = BufReader::new(file);
    let mut platforms = Vec::new();

    for line_base64 in reader.lines() {
        let line = STANDARD.decode(&line_base64?).unwrap();
        let decoded_entry = decode_entry(&line).unwrap();
        let platform = decoded_entry.platform;
        platforms.push(platform.to_string());
    }
    Ok(platforms)
}

pub fn add_new_value(
    file_path: &str,
    platform: &str,
    password: &str,
    master_password: &str,
) -> Result<()> {
    if !Path::new(file_path).exists() {
        File::create(file_path)?;
    }

    let path = Path::new(file_path);

    let file = OpenOptions::new().read(true).open(path)?;

    let reader = BufReader::new(file);

    let encoded_entry = encrypt_password(master_password, password, platform).unwrap();
    for line_base64 in reader.lines() {
        let line = STANDARD.decode(&line_base64?).unwrap();
        let decoded_entry = decode_entry(&line).unwrap();
        if platform == decoded_entry.platform {
            // Return an error if a duplicate name is found
            return Err(Error::new(
                ErrorKind::AlreadyExists,
                "Duplicate names: please use a new platform name",
            ));
        }
    }

    let mut file = OpenOptions::new().append(true).open(path)?;

    writeln!(file, "{}", STANDARD.encode(&encoded_entry))?;

    Ok(())
}

pub fn get_password(file_path: &str, platform: &str, master_password: &str) -> Result<String> {
    let mut password = String::new();
    if !Path::new(file_path).exists() {
        File::create(file_path)?;
    }

    let path = Path::new(file_path);

    let file = OpenOptions::new().read(true).open(path)?;

    let reader = BufReader::new(file);

    for line_base64 in reader.lines() {
        let line = STANDARD.decode(&line_base64?).unwrap();
        let decoded_entry = decode_entry(&line).unwrap();
        if decoded_entry.platform == platform {
            password = decrypt_password(master_password, decoded_entry).unwrap();
            break;
        }
    }

    if password.is_empty() {
        return Err(Error::new(
            ErrorKind::NotFound,
            "The platform does not exist, please create the platform before accessing it",
        ));
    }

    Ok(password)
}

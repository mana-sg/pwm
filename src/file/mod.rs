use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Error, ErrorKind, Result, Write};
use std::path::Path;

pub fn load_stored_password_platforms(file_path: &str) -> Result<Vec<String>> {
    if !Path::new(file_path).exists() {
        File::create(file_path)?;
    }

    let file = OpenOptions::new().read(true).open(file_path)?;

    let reader = BufReader::new(file);
    let mut names = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(colon_pos) = line.find(":") {
            let name = &line[0..colon_pos];
            names.push(name.to_string());
        }
    }
    Ok(names)
}

pub fn add_new_value(file_path: &str, name: &str, password: &str) -> Result<()> {
    if !Path::new(file_path).exists() {
        File::create(file_path)?;
    }

    let path = Path::new(file_path);

    let file = OpenOptions::new().read(true).open(path)?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Some(colon_pos) = line.find(":") {
            let name_in_line = &line[0..colon_pos];
            if name == name_in_line {
                // Return an error if a duplicate name is found
                return Err(Error::new(
                    ErrorKind::AlreadyExists,
                    "Duplicate names: please use a new platform name",
                ));
            }
        }
    }

    let mut file = OpenOptions::new().append(true).open(path)?;

    writeln!(file, "{}: {}", name, password)?;

    Ok(())
}

pub fn get_password(file_path: &str, name: &str) -> Result<String> {
    if !Path::new(file_path).exists() {
        File::create(file_path)?;
    }

    let path = Path::new(file_path);

    let file = OpenOptions::new().read(true).open(path)?;

    let reader = BufReader::new(file);
    let mut password = String::new();

    for line in reader.lines() {
        let line = line?;
        if let Some(colon_pos) = line.find(":") {
            let name_in_file = &line[0..colon_pos];
            if name_in_file == name {
                password = line[colon_pos + 2..].to_string();
                break;
            }
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

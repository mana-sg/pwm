mod crypto;
mod encoding;
mod file;
mod types;

use clap::{Arg, Command};
use std::io;

fn main() {
    let mut master_pwd = String::new();
    let file_path = "passwords.bin";
    let matches = Command::new("pwm")
        .version("0.1.0")
        .author("Manas github.com/mana-sg")
        .about("an application to manage your passwords")
        .subcommand(
            Command::new("add")
                .about("add a new password")
                .arg(Arg::new("name").required(true).help("name of the platform"))
                .arg(
                    Arg::new("password")
                        .required(true)
                        .help("password associated with the platform"),
                ),
        )
        .subcommand(Command::new("list").about("list all stored passwords"))
        .subcommand(
            Command::new("get")
                .about("retrieve password for mentioned platform")
                .arg(Arg::new("name").required(true).help("name of the platform")),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_matches)) => {
            if master_pwd.is_empty() {
                println!("Enter the master password: ");
                io::stdin().read_line(&mut master_pwd).unwrap();
            }
            let name = sub_matches.get_one::<String>("name").unwrap();
            let pwd = sub_matches.get_one::<String>("password").unwrap();

            let result = file::add_new_value(file_path, name, pwd, master_pwd.as_str());

            match result {
                Ok(_) => {
                    println!("password successfully added!\nName: {name}, Password: {pwd}");
                }
                Err(e) => {
                    eprintln!("error adding password: {}", e);
                }
            }
        }
        Some(("list", _)) => {
            let result = file::load_stored_password_platforms(file_path);

            match result {
                Ok(names) => {
                    if names.is_empty() {
                        println!("no stored passwords");
                    } else {
                        println!("stored platform passwords are: ");
                        for name in names {
                            println!("{}", name);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("error getting stored platforms: {}", e);
                }
            }
        }
        Some(("get", sub_matches)) => {
            if master_pwd.is_empty() {
                println!("Enter the master password: ");
                io::stdin().read_line(&mut master_pwd).unwrap();
            }
            let platform = sub_matches.get_one::<String>("name").unwrap();

            let result = file::get_password(file_path, platform, master_pwd.as_str());

            match result {
                Ok(password) => {
                    println!("{}", password);
                }
                Err(e) => {
                    eprintln!("error getting password: {}", e);
                }
            }
        }
        _ => {
            println!("please enter a valid choice!")
        }
    }
}

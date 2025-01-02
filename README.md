# pwm  
A Command-Line Password Manager in Rust  

**pwm** is a lightweight, secure, and offline password manager CLI tool. With one master password, you can securely store and retrieve all your platform-specific passwords.  

---

## Why I Built pwm  

I created **pwm** as a project to learn Rust and explore its powerful ecosystem. This tool demonstrates Rust's capabilities for secure, performant, and system-level programming.  

---

## How It Works  

**pwm** encrypts your passwords using AES-GCM, a robust encryption standard. A single master key is used to encrypt and decrypt all your passwords. Since the data resides on your local machine, you retain full control and security over your passwords.  

Each password is stored alongside a randomly generated nonce and is securely associated with the platform name. This ensures every password has a unique encryption key, even if the master password remains the same.  

---

## Installation  

To get started with **pwm**, ensure you have the following tools installed:  

- `git`  
- `cargo` (Rust package manager)  
- Rust compiler  
- `make`  

Then, follow these steps:  

```bash
# Clone the pwm repository
git clone https://github.com/mana-sg/pwm ~/pwm  

# Navigate to the pwm directory
cd ~/pwm  

# Build and install the pwm binary
make install  

# Verify installation
pwm -h  
```
---

## Usage
There are 3 main commands associated with pwm as of now:
1. `add`: to add new password platform pairs.
2. `get`: to get the password for a platform.
3. `list`: to list all added platforms.

### Here's how you use them
1. add
```bash
  pwm add <platform> <password>
```
Creates a platform password pair.

2. list
```bash
  pwm list
```
Lists all platforms that have been stored on the system.

3. get
```bash
  pwm get <platform>
```
Retrieves the password for the platform requested.

---

## Why pwm is Different

- **Local-Only Storage**: Your data stays on your machine—no cloud, no external servers.  
- **AES-GCM Encryption**: Advanced encryption to ensure your passwords remain secure.  
- **Rust-Powered**: Built with Rust, combining high performance with memory safety.  

Whether you're exploring Rust or looking for a simple, offline password manager, **pwm** is a great choice. Contributions and feedback are always welcome!

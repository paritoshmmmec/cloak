use dirs;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use toml;

const APP_DIR: &str = ".2fa/";
const RECOVERY_CODES_DIR: &str = "recovery_codes/";
const ACCOUNTS_FILE: &str = "accounts";

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    pub key: String,
    pub totp: bool,
    pub hash_function: String,
    pub counter: Option<u64>,
}

pub fn read() -> io::Result<BTreeMap<String, Account>> {
    let file_path = Path::new(&dirs::home_dir().unwrap())
        .join(APP_DIR)
        .join(ACCOUNTS_FILE);
    let accounts_str = fs::read_to_string(file_path)?;
    let accounts: BTreeMap<String, Account> = toml::from_str(&accounts_str).unwrap();
    Ok(accounts)
}

pub fn write(accounts: &BTreeMap<String, Account>) -> io::Result<()> {
    let file_path = Path::new(&dirs::home_dir().unwrap())
        .join(APP_DIR)
        .join(ACCOUNTS_FILE);
    let accounts_str = toml::to_string(accounts).unwrap();
    fs::write(file_path, accounts_str).expect("Unable to write file");
    Ok(())
}

pub fn recovery_codes(account_name: &str) -> io::Result<(PathBuf)> {
    let recovery_codes_dir = Path::new(&dirs::home_dir().unwrap())
        .join(APP_DIR)
        .join(RECOVERY_CODES_DIR);
    let file_path = recovery_codes_dir.join(account_name);
    fs::create_dir_all(&recovery_codes_dir)?;
    if !file_path.is_file() {
        let _ = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&file_path);
    }
    Ok(file_path)
}
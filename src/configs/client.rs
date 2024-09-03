use std::{
    fs, io,
    path::{Path, PathBuf},
    str::FromStr,
};

use serde::Deserialize;

fn default_session_file_path() -> PathBuf {
    PathBuf::from_str("configs/client.session").expect("Incorrect default config file path")
}

fn default_logging_directives() -> Box<str> {
    "info".to_owned().into_boxed_str()
}

#[derive(Deserialize)]
pub struct Client {
    pub api_id: i32,
    pub api_hash: String,

    password: Option<Box<str>>,
    #[serde(default = "default_session_file_path")]
    session_file_path: PathBuf,
    phone_number: Box<str>,
}

impl Client {
    pub fn password(&self) -> Option<&str> {
        match self.password {
            Some(ref password) => Some(password.trim()),
            None => None,
        }
    }

    pub fn session_file_path(&self) -> &Path {
        &self.session_file_path
    }

    pub fn phone_number(&self) -> &str {
        if self.phone_number.starts_with('+') {
            &self.phone_number[1..]
        } else {
            &self.phone_number
        }
    }
}

#[derive(Deserialize)]
pub struct Logging {
    #[serde(default = "default_logging_directives")]
    pub directives: Box<str>,
}

#[derive(Deserialize)]
pub struct Config {
    pub client: Client,
    pub logging: Logging,
}

impl Config {
    pub fn parse_raw_toml(raw: impl AsRef<str>) -> Result<Self, toml::de::Error> {
        toml::from_str(raw.as_ref())
    }
}

pub fn read_raw_toml(file_path: impl AsRef<Path>) -> Result<Box<str>, io::Error> {
    fs::read_to_string(file_path).map(String::into_boxed_str)
}

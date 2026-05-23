use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Result;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub bot: Bot,
    pub paths: Paths,
    pub server: Server,
}

impl Config {
    pub fn new() -> Self {
        Self {
            bot: Bot::new(),
            paths: Paths::new(),
            server: Server::new(),
        }
    }

    pub fn read_or_create(path: PathBuf) -> Result<Self> {
        let file = fs::read_to_string(&path);
        let mut config = Self::new();

        if let Ok(contents) = file {
            config = toml::from_str(&contents)?;
        } else {
            fs::write(&path, toml::to_string(&config)?)?;
        }

        Ok(config)
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Bot {
    pub token: String,
    pub date_format: String,
    pub timestamp_format: String,
}

impl Bot {
    fn new() -> Self {
        Self {
            token: String::default(),
            date_format: String::from("%d.%m.%Y"),
            timestamp_format: String::from("%d.%m.%Y %H:%M:%S"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Paths {
    pub database: String,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            database: String::from("db.json"),
        }
    }
}

impl Paths {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Server {
    pub guild: String,
    pub citations_channel: String,
    pub role_on_join: String,
    pub admins: Vec<String>,
}

impl Server {
    fn new() -> Self {
        Self::default()
    }
}

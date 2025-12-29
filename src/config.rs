use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};
use serenity::prelude::TypeMapKey;

use crate::error::Result;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub bot: Bot,
    pub paths: Paths,
    pub server: Server,
}

impl TypeMapKey for Config {
    type Value = Config;
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

#[derive(Serialize, Deserialize)]
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
            timestamp_format: String::from("%Y-%m-%d %H:%M:%S UTC"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Paths {
    pub birthdays: String,
    pub messages: String,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            birthdays: String::from("birthdays.csv"),
            messages: String::from("messages.txt"),
        }
    }
}

impl Paths {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Server {
    pub copy_channel: String,
    pub role_on_join: String,
    pub admins: Vec<String>,
}

impl Server {
    fn new() -> Self {
        Self::default()
    }
}

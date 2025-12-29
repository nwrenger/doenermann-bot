use serenity::all::{Colour, CreateEmbed};

use crate::ResponseContent;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// An expected option didn't resolve
    OptionResolve,
    /// Invalid Date
    InvalidDate(String),
    /// File System Error
    FileSystem(String),
    /// Toml Serialization
    Toml(String),
    /// Couldn't read the csv
    ReadCSV,
    /// Couldn't write to the csv
    WriteCSV,
    /// Other csv related error
    OtherCSV(String),
    /// Unathorized action
    Unauthorized,
    /// Command not found
    CommandNotFound,
}

impl Error {
    pub fn error_message(self) -> ResponseContent {
        let embed = match self {
            Error::OptionResolve => {
                CreateEmbed::default().title(String::from("Got invalid option!"))
            }
            Error::InvalidDate(e) => CreateEmbed::default().title(format!("Invalid Date: {e}!")),
            Error::FileSystem(e) => {
                CreateEmbed::default().title(format!("File System Error: {e}!"))
            }
            Error::Toml(e) => {
                CreateEmbed::default().title(format!("Toml Serialization Error: {e}!"))
            }
            Error::ReadCSV => {
                CreateEmbed::default().title(String::from("Couldn't read the CSV file!"))
            }
            Error::WriteCSV => {
                CreateEmbed::default().title(String::from("Couldn't write to the CSV file!"))
            }
            Error::OtherCSV(e) => CreateEmbed::default().title(format!("Unknown CSV Error: {e}!")),
            Error::Unauthorized => CreateEmbed::default()
                .title(String::from("You are unauthorized to do that action!")),
            Error::CommandNotFound => {
                CreateEmbed::default().title(String::from("Command not Found!"))
            }
        }
        .color(Colour::RED);

        ResponseContent::new_only_embed(embed)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::FileSystem(err.to_string())
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::Toml(err.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::Toml(err.to_string())
    }
}

impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Error {
        Error::InvalidDate(e.to_string())
    }
}

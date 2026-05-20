use serenity::all::{Colour, CreateEmbed, CreateInteractionResponseMessage};

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
    /// Reqwest Error
    Reqwest(String),
    /// Jikan API rate limit
    RateLimit,
    /// Jikan API error
    Jikan(String),
    /// Unathorized action
    Unauthorized,
    /// Command/Interaction not found
    NotFound,
}

impl Error {
    pub fn error_message(self) -> CreateInteractionResponseMessage {
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
            Error::Reqwest(e) => CreateEmbed::default().title(format!("Reqwest Error: {e}!")),
            Error::RateLimit => CreateEmbed::default().title(String::from(
                "API rate limit reached. Calm down and continue in a few seconds!",
            )),
            Error::Jikan(e) => CreateEmbed::default().title(format!("Jikan API Error: {e}!")),
            Error::Unauthorized => CreateEmbed::default()
                .title(String::from("You are unauthorized to do that action!")),
            Error::NotFound => {
                CreateEmbed::default().title(String::from("Command/Interaction not found!"))
            }
        }
        .color(Colour::RED);

        CreateInteractionResponseMessage::new()
            .add_embed(embed)
            .ephemeral(true)
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

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err.to_string())
    }
}

impl From<chrono::ParseError> for Error {
    fn from(e: chrono::ParseError) -> Error {
        Error::InvalidDate(e.to_string())
    }
}

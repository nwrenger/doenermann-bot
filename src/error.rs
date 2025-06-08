use serenity::all::CreateEmbed;

use crate::ResponseContent;

pub enum Error {
    /// An expected option didn't resolve
    OptionResolve,
    /// Invalid Date
    InvalidDate(String),
    /// Couldn't read the csv
    ReadCSV,
    /// Couldn't write to the csv
    WriteCSV,
    /// Other csv related error
    OtherCSV(String),
}

impl Error {
    pub fn error_message(self) -> ResponseContent {
        let embed = match self {
            Error::OptionResolve => {
                CreateEmbed::default().title(String::from("Got invalid option"))
            }
            Error::InvalidDate(e) => CreateEmbed::default().title(format!("Invalid date, {e}")),
            Error::ReadCSV => {
                CreateEmbed::default().title(String::from("Couldn't read the CSV file"))
            }
            Error::WriteCSV => {
                CreateEmbed::default().title(String::from("Couldn't write to the CSV file"))
            }
            Error::OtherCSV(e) => CreateEmbed::default().title(e),
        };

        ResponseContent::new_only_embed(embed)
    }
}

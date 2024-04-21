use chrono::{Local, NaiveDate};
use csv::{Reader, Writer};
use serde::{self};
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed, ResolvedOption,
};

use crate::ResponseContent;

#[derive(serde::Serialize, serde::Deserialize, Default)]
struct Row {
    birthday: String,
    user: u64,
}

pub fn run(options: &[ResolvedOption], user: u64) -> ResponseContent {
    let date_fmt = "%d.%m.%Y";
    let embed = match options.first() {
        Some(option) => {
            let value = match option.value {
                serenity::all::ResolvedValue::String(str) => str,
                _ => "",
            };

            let date = if let Ok(date) = NaiveDate::parse_from_str(value, date_fmt) {
                if Local::now().date_naive().years_since(date).is_some() {
                    Some(date)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(date) = date {
                let rdr = Reader::from_path("birthdays.csv");
                if let Ok(mut rdr) = rdr {
                    let mut rows: Vec<Row> = rdr
                        .deserialize()
                        .map(|result| result.unwrap_or_default())
                        .collect();

                    // Update the user's birthday or add a new row if it doesn't exist
                    let mut found = false;
                    for row in rows.iter_mut() {
                        if row.user == user {
                            row.birthday = date.to_string();
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        rows.push(Row {
                            birthday: date.to_string(),
                            user,
                        });
                    }

                    // Write the updated data back to the CSV file
                    let wtr = Writer::from_path("birthdays.csv");
                    if let Ok(mut wtr) = wtr {
                        for row in rows.iter() {
                            wtr.serialize(row).unwrap_or_default()
                        }
                        if let Err(e) = wtr.flush() {
                            CreateEmbed::default()
                                .title("An error occurred: ".to_string() + &e.to_string())
                        } else {
                            CreateEmbed::default()
                                .title("Your Birthday was set to: ".to_string() + value)
                        }
                    } else {
                        CreateEmbed::default()
                            .title("An Error occurred: Couldn't write to CSV File".to_string())
                    }
                } else {
                    CreateEmbed::default()
                        .title("An Error occurred: Couldn't read CSV File".to_string())
                }
            } else {
                CreateEmbed::default().title("Invalid date: ".to_string() + value)
            }
        }
        _ => CreateEmbed::default().title("Expected Option, nothing given!".to_string()),
    };
    ResponseContent {
        text: "".to_string(),
        embed,
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("set_birthday")
        .description("Set your Birhtday")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "birth",
                "Format: Day.Month.Year like 02.02.2007",
            )
            .required(true),
        )
}

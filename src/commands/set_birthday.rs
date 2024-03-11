use chrono::{Datelike, Local, NaiveDate};
use csv::{Reader, Writer};
use serde::{self};
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed, ResolvedOption,
};

use crate::ResponseContent;

#[derive(serde::Serialize, serde::Deserialize)]
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
            if NaiveDate::parse_from_str(value, date_fmt).is_ok()
                && Local::now()
                    .date_naive()
                    .years_since(NaiveDate::parse_from_str(value, date_fmt).unwrap())
                    .is_some()
            {
                let mut rdr = Reader::from_path("birthdays.csv").unwrap();
                let mut rows: Vec<Row> = rdr.deserialize().map(|result| result.unwrap()).collect();

                // Update the user's birthday or add a new row if it doesn't exist
                let date = NaiveDate::parse_from_str(value, date_fmt).unwrap();
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
                let mut wtr = Writer::from_path("birthdays.csv").unwrap();
                for row in rows.iter() {
                    wtr.serialize(row).unwrap();
                }
                wtr.flush().unwrap();

                CreateEmbed::default().title("Your Birthday was set to: ".to_string() + value)
            } else if Local::now()
                .date_naive()
                .years_since(NaiveDate::parse_from_str(value, date_fmt).unwrap())
                .is_some()
            {
                CreateEmbed::default().title("Invalid Date!").description(
                    NaiveDate::parse_from_str(value, date_fmt)
                        .err()
                        .unwrap()
                        .to_string(),
                )
            } else {
                CreateEmbed::default()
                    .title("Invalid Date!")
                    .description(format!(
                        "Stop it Erik! You can't set Dates over the current Year! ({})",
                        Local::now().date_naive().year()
                    ))
            }
        }
        _ => panic!("Expected String type"),
    };
    ResponseContent {
        text: "".to_string(),
        embed,
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("set_birthday")
        .description("Set your Birhtday")
        .add_option(CreateCommandOption::new(
            CommandOptionType::String,
            "birth",
            "Format: Day.Month.Year like 02.02.2007",
        ))
}

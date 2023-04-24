use chrono::NaiveDate;
use csv::{Writer, Reader};
use serde;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

#[derive(serde::Serialize, serde::Deserialize)]
struct Row {
    birthday: String,
    user: u64,
}

pub fn run(options: &[CommandDataOption], user: u64) -> (String, CreateEmbed) {
    let mut embed = CreateEmbed::default();
    match options.get(0) {
        Some(option) => match &option.resolved {
            Some(CommandDataOptionValue::String(value)) => {
                if NaiveDate::parse_from_str(value, "%d.%m.%Y").is_ok() {
                    let mut rdr = Reader::from_path("birthdays.csv").unwrap();
                    let mut rows: Vec<Row> = rdr
                        .deserialize()
                        .map(|result| result.unwrap())
                        .collect();

                    // Update the user's birthday or add a new row if it doesn't exist
                    let date = NaiveDate::parse_from_str(value, "%d.%m.%Y").unwrap();
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

                    embed.title("Your Birthday was set to: ".to_string() + value)
                } else {
                    embed.title("Invalid Date!");
                    embed.description(
                        &NaiveDate::parse_from_str(value, "%d.%m.%Y")
                            .err()
                            .unwrap()
                            .to_string(),
                    )
                }
            }
            _ => panic!("Expected String type"),
        },
        None => panic!("Expected user option"),
    };
    ("".to_string(), embed)
}

pub fn _register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("set_birhday")
        .description("Set your Birhtday")
        .create_option(|f| {
            f.name("birth")
                .description("Format: Day.Month.Year like 02.02.2007")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

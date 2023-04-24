use chrono::NaiveDate;
use csv::Writer;
use serde;
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

#[derive(serde::Serialize)]
struct Row {
    birthday: String,
    user: u64,
}

pub fn run(options: &[CommandDataOption], user: u64) -> (String, CreateEmbed) {
    let mut embed = CreateEmbed::default();
    match options.get(0) {
        Some(option) => match &option.resolved {
            Some(CommandDataOptionValue::String(value)) => {
                if NaiveDate::parse_from_str(&value, "%d.%m.%Y").is_ok() {
                    let mut wtr = Writer::from_path("birthdays.csv").unwrap();
                    let date = NaiveDate::parse_from_str(&value, "%d.%m.%Y").unwrap();
                    wtr.serialize(Row {
                        birthday: date.to_string(),
                        user,
                    })
                    .unwrap();
                    embed.title("Your Birthday was set to: ".to_string() + &value)
                } else {
                    embed.title("Invalid Date!");
                    embed.description(
                        &NaiveDate::parse_from_str(&value, "%d.%m.%Y")
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

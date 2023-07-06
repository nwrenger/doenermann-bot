use chrono::{Datelike, Local, NaiveDate};
use csv::{Reader, Writer};
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
    let date_fmt = "%d.%m.%Y";
    match options.get(0) {
        Some(option) => match &option.resolved {
            Some(CommandDataOptionValue::String(value)) => {
                if NaiveDate::parse_from_str(value, date_fmt).is_ok()
                    && Local::now()
                        .date_naive()
                        .years_since(NaiveDate::parse_from_str(value, date_fmt).unwrap())
                        .is_some()
                {
                    let mut rdr = Reader::from_path("birthdays.csv").unwrap();
                    let mut rows: Vec<Row> =
                        rdr.deserialize().map(|result| result.unwrap()).collect();

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

                    embed.title("Your Birthday was set to: ".to_string() + value)
                } else {
                    embed.title("Invalid Date!");
                    if Local::now()
                        .date_naive()
                        .years_since(NaiveDate::parse_from_str(value, date_fmt).unwrap())
                        .is_some()
                    {
                        embed.description(
                            &NaiveDate::parse_from_str(value, date_fmt)
                                .err()
                                .unwrap()
                                .to_string(),
                        )
                    } else {
                        embed.description(format!(
                            "Stop it Erik! You can't set Dates over the current Year! ({})",
                            Local::now().date_naive().year()
                        ))
                    }
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
        .name("set_birthday")
        .description("Set your Birhtday")
        .create_option(|f| {
            f.name("birth")
                .description("Format: Day.Month.Year like 02.02.2007")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

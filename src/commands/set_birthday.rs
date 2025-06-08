use chrono::{Local, NaiveDate};
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed, ResolvedOption,
};

use crate::{
    commands::{load_birthdays, save_birthdays, BirthdayRow},
    error::Error,
    ResponseContent,
};

pub fn run(options: &[ResolvedOption], user: u64) -> ResponseContent {
    let date_fmt = "%d.%m.%Y";
    let year_option = &options[0];
    let value = match year_option.value {
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
        match load_birthdays() {
            Ok(mut rows) => {
                let mut found = false;
                for row in rows.iter_mut() {
                    if row.user == user {
                        row.birthday = date.to_string();
                        found = true;
                        break;
                    }
                }
                if !found {
                    rows.push(BirthdayRow {
                        birthday: date.to_string(),
                        user,
                    });
                }

                if let Err(e) = save_birthdays(&rows) {
                    return e.error_message();
                }

                ResponseContent::new_only_embed(
                    CreateEmbed::default().title(format!("Your Birthday was set to: {value}")),
                )
            }
            Err(e) => e.error_message(),
        }
    } else {
        Error::InvalidDate(value.to_string()).error_message()
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

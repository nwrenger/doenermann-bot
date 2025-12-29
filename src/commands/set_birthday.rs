use chrono::{Local, NaiveDate};
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed, ResolvedOption,
    ResolvedValue,
};

use crate::{
    commands::{load_birthdays, save_birthdays, BirthdayRow},
    config::Config,
    error::{Error, Result},
    ResponseContent,
};

pub fn run(options: &[ResolvedOption], config: &Config, user: u64) -> Result<ResponseContent> {
    let year_option = &options[0];

    if let ResolvedValue::String(value) = year_option.value {
        let parsed_date = NaiveDate::parse_from_str(value, &config.bot.date_format)?;
        let date = if Local::now().date_naive().years_since(parsed_date).is_some() {
            parsed_date
        } else {
            return Err(Error::InvalidDate(String::from(
                "You cannot go back in time",
            )));
        };

        let mut rows = load_birthdays(&config.paths.birthdays)?;
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

        save_birthdays(&config.paths.birthdays, &rows)?;

        Ok(ResponseContent::new_only_embed(
            CreateEmbed::default().title(format!("Your Birthday was set to: {value}")),
        ))
    } else {
        Err(Error::OptionResolve)
    }
}

pub fn register(config: &Config) -> CreateCommand {
    CreateCommand::new("set_birthday")
        .description("Set your birthday date")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "birth",
                format!("Date format: {}", &config.bot.date_format),
            )
            .required(true),
        )
}

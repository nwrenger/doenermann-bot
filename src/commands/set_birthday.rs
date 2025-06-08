use chrono::{Local, NaiveDate};
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed, ResolvedOption,
};

use crate::{
    commands::{load_birthdays, save_birthdays, BirthdayRow},
    error::{Error, Result},
    ResponseContent,
};

pub fn run(options: &[ResolvedOption], user: u64) -> Result<ResponseContent> {
    let date_fmt = "%d.%m.%Y";
    let year_option = &options[0];
    let value = match year_option.value {
        serenity::all::ResolvedValue::String(str) => str,
        _ => "",
    };

    let parsed_date = NaiveDate::parse_from_str(value, date_fmt)?;
    let date = if Local::now().date_naive().years_since(parsed_date).is_some() {
        parsed_date
    } else {
        return Err(Error::InvalidDate(String::from(
            "You cannot go back in time",
        )));
    };

    let mut rows = load_birthdays()?;
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

    save_birthdays(&rows)?;

    Ok(ResponseContent::new_only_embed(
        CreateEmbed::default().title(format!("Your Birthday was set to: {value}")),
    ))
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

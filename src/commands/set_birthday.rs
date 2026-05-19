use std::sync::Arc;

use chrono::{Local, NaiveDate};
use light_magic::atomic::AtomicDatabase;
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed, ResolvedOption,
    ResolvedValue,
};

use crate::{
    db::{Birthday, Database, User},
    error::{Error, Result},
    util::ResponseContent,
};

pub fn run(
    options: &[ResolvedOption],
    db: Arc<AtomicDatabase<Database>>,
    user: User,
) -> Result<ResponseContent> {
    let year_option = &options[0];

    if let ResolvedValue::String(value) = year_option.value {
        let parsed_date = {
            let db = db.read();
            NaiveDate::parse_from_str(value, &db.config.bot.date_format)?
        };
        let date = if Local::now().date_naive().years_since(parsed_date).is_some() {
            parsed_date
        } else {
            return Err(Error::InvalidDate(String::from(
                "You cannot go back in time",
            )));
        };

        db.write().birthdays.add(Birthday::new(user, date));

        Ok(ResponseContent::new_only_embed(
            CreateEmbed::default().title(format!("Your Birthday was set to: {value}")),
        ))
    } else {
        Err(Error::OptionResolve)
    }
}

pub fn register(date_format: String) -> CreateCommand {
    CreateCommand::new("set_birthday")
        .description("Set your birthday date")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "birth",
                format!("Date format: {}", date_format),
            )
            .required(true),
        )
}

use std::sync::Arc;

use chrono::{Local, NaiveDate};
use light_magic::atomic::AtomicDatabase;
use serenity::all::{
    CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage, ResolvedOption,
    ResolvedValue,
};

use crate::{
    db::{Birthday, Database},
    error::{Error, Result},
};

pub fn run(
    options: &[ResolvedOption],
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
    date_format: &str,
) -> Result<CreateInteractionResponse<'static>> {
    let year_option = options.first().ok_or(Error::OptionResolve)?;

    if let ResolvedValue::String(value) = year_option.value {
        let parsed_date = NaiveDate::parse_from_str(value, date_format)?;
        let date = if Local::now().date_naive().years_since(parsed_date).is_some() {
            parsed_date
        } else {
            return Err(Error::InvalidDate(String::from(
                "You cannot go back in time",
            )));
        };

        db.write().birthdays.add(Birthday::new(user_id, date));

        Ok(CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .embed(CreateEmbed::default().title(format!("Your Birthday was set to: {value}"))),
        ))
    } else {
        Err(Error::OptionResolve)
    }
}

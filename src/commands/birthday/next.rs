use std::sync::Arc;

use chrono::{Datelike, Local};
use light_magic::atomic::AtomicDatabase;
use serenity::all::CreateInteractionResponseMessage;
use serenity::builder::CreateEmbed;

use crate::db::Database;
use crate::error::Result;
use crate::util::EMPTY_FIELD_VALUE;

const MAX_BIRTHDAY_FIELDS: usize = 10;
const FUTURE_FORMAT: &str = "%d %B %Y";

pub fn run(db: Arc<AtomicDatabase<Database>>) -> Result<CreateInteractionResponseMessage> {
    let mut birthdays = db.read().birthdays.values().cloned().collect::<Vec<_>>();
    let now = Local::now().date_naive();

    birthdays.sort_by_key(|birthday| {
        let date = birthday.date;
        let next_date = if date.with_year(now.year()) < now.with_year(now.year()) {
            date.with_year(now.year() + 1).unwrap_or_default()
        } else {
            date.with_year(now.year()).unwrap_or_default()
        };
        next_date.signed_duration_since(now).num_days().abs()
    });

    let mut embed = CreateEmbed::default().title("Next Birthdays:");

    let length = if birthdays.len() < MAX_BIRTHDAY_FIELDS {
        birthdays.len()
    } else {
        MAX_BIRTHDAY_FIELDS
    };

    for birthday in birthdays.drain(..length) {
        let date = birthday.date;
        let future = if date.with_year(now.year()) < now.with_year(now.year()) {
            date.with_year(now.year() + 1).unwrap_or_default()
        } else {
            date.with_year(now.year()).unwrap_or_default()
        };
        let age = if date.with_year(now.year()) == now.with_year(now.year()) {
            now.years_since(date).unwrap_or_default()
        } else {
            now.years_since(date).unwrap_or_default() + 1
        };
        embed = embed.field(
            future.format(FUTURE_FORMAT).to_string(),
            format!("<@{}> ({})", birthday.user_id, age),
            false,
        );
    }

    if length == 0 {
        embed = embed.field(EMPTY_FIELD_VALUE, String::from("None recorded yet!"), false);
    }

    Ok(CreateInteractionResponseMessage::new().embed(embed))
}

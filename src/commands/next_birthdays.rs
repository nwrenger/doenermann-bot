use chrono::{Datelike, Local, NaiveDate};
use serenity::{
    all::{CreateCommand, ResolvedOption},
    builder::CreateEmbed,
};

use crate::error::Result;
use crate::{commands::load_birthdays, ResponseContent};

pub fn run(_options: &[ResolvedOption]) -> Result<ResponseContent> {
    let mut rows = load_birthdays()?;
    let now = Local::now().date_naive();

    rows.sort_by_key(|row| {
        let birthday = NaiveDate::parse_from_str(&row.birthday, "%Y-%m-%d").unwrap_or_default();
        let next_birthday = if birthday.with_year(now.year()) < now.with_year(now.year()) {
            birthday.with_year(now.year() + 1).unwrap_or_default()
        } else {
            birthday.with_year(now.year()).unwrap_or_default()
        };
        next_birthday.signed_duration_since(now).num_days().abs()
    });

    let mut embed = CreateEmbed::default().title("Next Birthdays:");

    for i in rows.drain(..10) {
        let date = NaiveDate::parse_from_str(&i.birthday, "%Y-%m-%d")?;
        let future = if date.with_year(now.year()) < now.with_year(now.year()) {
            date.with_year(now.year() + 1).unwrap_or_default()
        } else {
            date.with_year(now.year()).unwrap_or_default()
        };
        let age = if date.with_year(now.year()) == now.with_year(now.year()) {
            Local::now()
                .date_naive()
                .years_since(date)
                .unwrap_or_default()
        } else {
            Local::now()
                .date_naive()
                .years_since(date)
                .unwrap_or_default()
                + 1
        };
        embed = embed.field(
            future.format("%d %B %Y").to_string(),
            format!("<@{}> ({})", i.user, age),
            false,
        );
    }

    Ok(ResponseContent::new_only_embed(embed))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("next_birthdays").description("The next 10 Upcomming Birthdays")
}

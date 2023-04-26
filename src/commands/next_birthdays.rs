use csv::Reader;
use chrono::{NaiveDate, Local, Datelike};
use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::CommandDataOption;

#[derive(serde::Serialize, serde::Deserialize)]
struct Row {
    birthday: String,
    user: u64,
}

pub fn run(_options: &[CommandDataOption]) -> (String, CreateEmbed) {
    let mut embed = CreateEmbed::default();
    embed.title("Next Birthdays:");
    let mut rdr = Reader::from_path("birthdays.csv").unwrap();
    let mut rows: Vec<Row> =
        rdr.deserialize().map(|result| result.unwrap()).collect();
    let now = Local::now().date_naive();
    
    rows.sort_by_key(|row| {
        let birthday = NaiveDate::parse_from_str(&row.birthday, "%Y-%m-%d").unwrap();
        let next_birthday = if birthday.with_year(now.year()) < now.with_year(now.year()) {
            birthday.with_year(now.year() + 1).unwrap()
        } else {
            birthday.with_year(now.year()).unwrap()
        };
        next_birthday.signed_duration_since(now).num_days().abs()
    });

    for i in rows.drain(..10) {
        let date = NaiveDate::parse_from_str(&i.birthday, "%Y-%m-%d").unwrap();
        let future = if date.with_year(now.year()) < now.with_year(now.year()) {
            date.with_year(now.year() + 1).unwrap()
        } else {
            date.with_year(now.year()).unwrap()
        };
        let age = if date.with_year(now.year()) == now.with_year(now.year()) {
            Local::now().date_naive().years_since(date).unwrap()
        } else {
            Local::now().date_naive().years_since(date).unwrap() + 1
        };
        embed.field(
            future.format("%d %B %Y").to_string(),
            format!("<@{}> ({})", i.user, age),
            false,
        );
    }
    ("".to_string(), embed)
}

pub fn _register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("next_birthdays")
        .description("The next 10 Upcomming Birthdays")
}

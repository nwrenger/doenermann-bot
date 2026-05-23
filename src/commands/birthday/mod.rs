pub mod delete;
pub mod next;
pub mod set;

use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, CreateInteractionResponse,
    ResolvedOption, ResolvedValue,
};

use crate::{
    db::Database,
    error::{Error, Result},
};

pub fn run(
    options: &[ResolvedOption],
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
    date_format: &str,
    admins: &[String],
) -> Result<CreateInteractionResponse<'static>> {
    let subcommand = options.first().ok_or(Error::OptionResolve)?;

    if let ResolvedValue::SubCommand(options) = &subcommand.value {
        match subcommand.name {
            "set" => set::run(options, db, user_id, date_format),
            "delete" => delete::run(options, db, user_id, admins),
            "next" => next::run(db),
            _ => Err(Error::NotFound),
        }
    } else {
        Err(Error::OptionResolve)
    }
}

pub fn register(date_format: String) -> CreateCommand<'static> {
    CreateCommand::new("birthday")
        .description("Manage birthdays")
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "set", "Set your birthday")
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "birth",
                        format!("Date format: {}", date_format),
                    )
                    .required(true),
                ),
        )
        .add_option(
            CreateCommandOption::new(CommandOptionType::SubCommand, "delete", "Delete a birthday")
                .add_sub_option(
                    CreateCommandOption::new(
                        CommandOptionType::User,
                        "user",
                        "Select yourself, or another user if you are an admin",
                    )
                    .required(true),
                ),
        )
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "next",
            "Show the next 10 upcoming birthdays",
        ))
}

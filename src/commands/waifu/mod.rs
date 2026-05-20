pub mod collection;
pub mod roll;

use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};

use crate::{
    db::Database,
    error::{Error, Result},
    util::ResponseContent,
};

pub fn run(
    options: &[ResolvedOption],
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
) -> Result<ResponseContent> {
    let subcommand = options.first().ok_or(Error::OptionResolve)?;

    if let ResolvedValue::SubCommand(_) = &subcommand.value {
        match subcommand.name {
            "collection" => collection::run(db, user_id),
            "roll" => roll::run(db),
            _ => Err(Error::CommandNotFound),
        }
    } else {
        Err(Error::OptionResolve)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("waifu")
        .description("Manage waifus")
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "collection",
            "See your waifu collection",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "roll",
            "Roll for new waifus",
        ))
}

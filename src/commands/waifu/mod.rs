pub mod collection;
pub mod leaderboard;
pub mod roll;

use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, CreateInteractionResponseMessage,
    ResolvedOption, ResolvedValue,
};

use crate::{
    db::Database,
    error::{Error, Result},
};

pub async fn run<'a>(
    options: &'a [ResolvedOption<'a>],
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
) -> Result<CreateInteractionResponseMessage> {
    let subcommand = options.first().ok_or(Error::OptionResolve)?;

    if let ResolvedValue::SubCommand(_) = &subcommand.value {
        match subcommand.name {
            "collection" => collection::run(db, user_id, None),
            "leaderboard" => leaderboard::run(db),
            "roll" => roll::run().await,
            _ => Err(Error::NotFound),
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
            "leaderboard",
            "See the waifu leaderboard",
        ))
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "roll",
            "Roll for new waifus",
        ))
}

pub mod collection;
pub mod leaderboard;
pub mod roll;

use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, CreateEmbed, CreateInteractionResponse,
    ResolvedOption, ResolvedValue,
};

use crate::{
    api::MAL_PAGE,
    db::{Character, Database},
    error::{Error, Result},
    util::color_goon_credits,
};

pub async fn run<'a>(
    options: &[ResolvedOption<'a>],
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
) -> Result<CreateInteractionResponse<'static>> {
    let subcommand = options.first().ok_or(Error::OptionResolve)?;

    if let ResolvedValue::SubCommand(_) = &subcommand.value {
        match subcommand.name {
            "collection" => collection::run(db, user_id, None),
            "leaderboard" => leaderboard::run(db),
            "roll" => roll::run(db).await,
            _ => Err(Error::NotFound),
        }
    } else {
        Err(Error::OptionResolve)
    }
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("waifu")
        .description("Manage waifus")
        .add_option(CreateCommandOption::new(
            CommandOptionType::SubCommand,
            "collection",
            "Manage your waifu collection",
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

pub fn create_character_embed(character: &Character) -> CreateEmbed<'static> {
    let mut embed = CreateEmbed::default()
        .title(character.name.clone())
        .field("Goon Credits", character.goon_credits.to_string(), true)
        .url(format!("{}/{}", MAL_PAGE, character.mal_id));

    if let Some(image_url) = character.image_url() {
        embed = embed.image(image_url.to_string());
    }

    if let Some(color) = color_goon_credits(character.goon_credits) {
        embed = embed.color(color);
    }

    embed
}

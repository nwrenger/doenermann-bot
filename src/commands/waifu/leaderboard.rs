use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::{CreateEmbed, CreateInteractionResponseMessage};

use crate::db::Database;
use crate::error::Result;

const MAX_LEADERBOARD_FIELDS: usize = 25;

pub fn run(db: Arc<AtomicDatabase<Database>>) -> Result<CreateInteractionResponseMessage> {
    let mut players = db
        .read()
        .collections
        .values()
        .map(|collection| {
            let goon_credits = collection
                .characters
                .values()
                .map(|character| character.goon_credits as u64)
                .sum::<u64>();

            (
                collection.user_id,
                goon_credits,
                collection.characters.values().len(),
            )
        })
        .collect::<Vec<_>>();

    players.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.2.cmp(&a.2)));

    let mut embed = CreateEmbed::new().title("Waifu Leaderboard");

    if players.is_empty() {
        embed = embed.description("No waifus have been claimed yet!");
    } else {
        for (rank, (user_id, goon_credits, character_count)) in
            players.iter().take(MAX_LEADERBOARD_FIELDS).enumerate()
        {
            embed = embed.field(
                format!("#{}", rank + 1),
                format!(
                    "<@{}> has {} Goon Credits across {} {}",
                    user_id,
                    goon_credits,
                    character_count,
                    waifu_label(*character_count)
                ),
                false,
            );
        }
    }

    Ok(CreateInteractionResponseMessage::new().embed(embed))
}

fn waifu_label(count: usize) -> &'static str {
    if count == 1 {
        "waifu"
    } else {
        "waifus"
    }
}

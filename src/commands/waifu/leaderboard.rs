use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::{CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

use crate::db::Database;
use crate::error::Result;

const MAX_LEADERBOARD_FIELDS: usize = 25;

pub fn run(db: Arc<AtomicDatabase<Database>>) -> Result<CreateInteractionResponse<'static>> {
    let (mut players, total_goon_credits, roll_count) = {
        let db = db.read();
        let players = db
            .collections
            .values()
            .map(|collection| {
                let goon_credits = collection
                    .characters
                    .iter()
                    .map(|character| character.goon_credits as u64)
                    .sum::<u64>();

                (
                    collection.user_id,
                    goon_credits,
                    collection.characters.len(),
                )
            })
            .collect::<Vec<_>>();
        let total_goon_credits: u64 = players
            .iter()
            .map(|(_, goon_credits, _)| goon_credits)
            .sum();

        (players, total_goon_credits, db.roll_count)
    };

    players.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| b.2.cmp(&a.2)));

    let mut embed = CreateEmbed::new()
        .title("Waifu Leaderboard:")
        .description(format!(
            "Total Goon Credits: {} across {} rolls",
            total_goon_credits, roll_count
        ));

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

    Ok(CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().embed(embed),
    ))
}

fn waifu_label(count: usize) -> &'static str {
    if count == 1 {
        "waifu"
    } else {
        "waifus"
    }
}

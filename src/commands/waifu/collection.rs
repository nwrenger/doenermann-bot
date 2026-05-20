use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::{
    ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponseMessage,
};

use crate::db::Database;
use crate::error::{Error, Result};
use crate::util::color_goon_credits;

pub fn run(
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
    mal_id: Option<u32>,
) -> Result<CreateInteractionResponseMessage> {
    if let Some(collection) = db.read().collections.get(&user_id) {
        let characters = collection.characters.values().cloned().collect::<Vec<_>>();
        let current_index = mal_id
            .and_then(|mal_id| {
                characters
                    .iter()
                    .position(|character| character.mal_id == mal_id)
            })
            .unwrap_or_default();

        if let Some(current) = characters.get(current_index) {
            let mut embed = CreateEmbed::default()
                .title(&current.name)
                .field("Goon Credits", current.goon_credits.to_string(), true)
                .image(current.image.to_string());

            if let Some(color) = color_goon_credits(current.goon_credits) {
                embed = embed.color(color);
            }

            let previous_id = current_index
                .checked_sub(1)
                .and_then(|index| characters.get(index))
                .map(|character| character.mal_id);
            let next_id = characters
                .get(current_index + 1)
                .map(|character| character.mal_id);

            let buttons = CreateActionRow::Buttons(vec![
                CreateButton::new(format!(
                    "previous:{}",
                    previous_id.unwrap_or(current.mal_id)
                ))
                .label("Previous")
                .style(ButtonStyle::Secondary)
                .disabled(previous_id.is_none()),
                CreateButton::new(format!("delete:{},{}", user_id, current.mal_id))
                    .label("Delete")
                    .style(ButtonStyle::Danger),
                CreateButton::new(format!("next:{}", next_id.unwrap_or(current.mal_id)))
                    .label("Next")
                    .style(ButtonStyle::Secondary)
                    .disabled(next_id.is_none()),
            ]);

            return Ok(CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(vec![buttons]));
        }
    }

    Ok(CreateInteractionResponseMessage::new()
        .embed(CreateEmbed::new().title("Your collection is empty!"))
        .components(vec![]))
}

pub fn delete(
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
    owner_id: u64,
    mal_id: u32,
) -> Result<CreateInteractionResponseMessage> {
    if user_id == owner_id {
        if let Some(collection) = db.write().collections.get_mut(&owner_id) {
            collection.characters.delete(&mal_id);
        }

        run(db, owner_id, None)
    } else {
        Err(Error::Unauthorized)
    }
}

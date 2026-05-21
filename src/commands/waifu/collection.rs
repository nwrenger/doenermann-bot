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
    index: Option<usize>,
) -> Result<CreateInteractionResponseMessage> {
    if let Some(collection) = db.read().collections.get(&user_id) {
        let current_index = index.unwrap_or_default();

        if let Some(current) = collection.characters.get(current_index) {
            let mut embed = CreateEmbed::default()
                .title(&current.name)
                .field("Goon Credits", current.goon_credits.to_string(), true)
                .image(current.image.to_string());

            if let Some(color) = color_goon_credits(current.goon_credits) {
                embed = embed.color(color);
            }

            let previous_id = current_index
                .checked_sub(1)
                .filter(|index| collection.characters.get(*index).is_some());
            let next_id = collection
                .characters
                .get(current_index + 1)
                .map(|_| current_index + 1);

            let buttons = CreateActionRow::Buttons(vec![
                CreateButton::new(format!(
                    "previous:{},{}",
                    user_id,
                    previous_id.unwrap_or(current_index)
                ))
                .label("Previous")
                .style(ButtonStyle::Secondary)
                .disabled(previous_id.is_none()),
                CreateButton::new(format!("delete:{},{}", user_id, current_index))
                    .label("Delete")
                    .style(ButtonStyle::Danger),
                CreateButton::new(format!(
                    "next:{},{}",
                    user_id,
                    next_id.unwrap_or(current_index)
                ))
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
    index: usize,
) -> Result<CreateInteractionResponseMessage> {
    if user_id == owner_id {
        let previous_index = {
            let mut db = db.write();

            db.collections.get_mut(&owner_id).and_then(|collection| {
                collection.characters.get(index)?;
                let previous_index = index.checked_sub(1);

                collection.characters.remove(index);

                previous_index
            })
        };

        run(db, owner_id, previous_index)
    } else {
        Err(Error::Unauthorized)
    }
}

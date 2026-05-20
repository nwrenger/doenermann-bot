use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::{
    ButtonStyle, CreateActionRow, CreateButton, CreateEmbed, CreateInteractionResponseMessage,
};

use crate::api::get_random_character;
use crate::db::{Character, Collection, Database};
use crate::error::Result;
use crate::util::color_goon_credits;

pub async fn run() -> Result<CreateInteractionResponseMessage> {
    let character = get_random_character().await?;

    let mut embed = CreateEmbed::default()
        .title(&character.name)
        .field("Goon Credits", character.goon_credits.to_string(), true)
        .image(character.image.to_string());

    if let Some(color) = color_goon_credits(character.goon_credits) {
        embed = embed.color(color);
    }

    let claim = CreateActionRow::Buttons(vec![CreateButton::new(format!(
        "claim:{},{},{},{}",
        character.mal_id, &character.name, character.goon_credits, character.image
    ))
    .label("Claim")
    .style(ButtonStyle::Secondary)]);

    Ok(CreateInteractionResponseMessage::new()
        .embed(embed)
        .components(vec![claim]))
}

pub fn claim(
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
    user_name: &str,
    character: Character,
) -> Result<CreateInteractionResponseMessage> {
    let mut db = db.write();

    let claimed_by = db
        .collections
        .values()
        .find(|collection| collection.characters.get(&character.mal_id).is_some())
        .map(|collection| collection.user_name.clone());

    if let Some(user_name) = claimed_by {
        return Ok(claimed_message(&user_name));
    }

    if let Some(collection) = db.collections.get_mut(&user_id) {
        collection.user_name = user_name.to_string();
        collection.characters.add(character);
    } else {
        let mut collection = Collection::new(user_id, user_name.to_string());
        collection.characters.add(character);
        db.collections.add(collection);
    }

    Ok(claimed_message(user_name))
}

fn claimed_message(user_name: &str) -> CreateInteractionResponseMessage {
    CreateInteractionResponseMessage::new().components(vec![CreateActionRow::Buttons(vec![
        CreateButton::new("claimed")
            .label(format!("Claimed by {}!", user_name))
            .style(ButtonStyle::Secondary)
            .disabled(true),
    ])])
}

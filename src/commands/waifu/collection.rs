use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::{
    ButtonStyle, CommandDataResolved, ComponentInteractionDataKind, CreateActionRow, CreateButton,
    CreateComponent, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage,
    CreateLabel, CreateModal, CreateModalComponent, CreateSelectMenu, CreateSelectMenuKind,
    CreateSelectMenuOption,
};

use crate::commands::waifu::create_character_embed;
use crate::db::{Character, Collection, Database};
use crate::error::{Error, Result};
use crate::util::truncate_chars;

const SELECT_PAGE_SIZE: usize = 23;
const MODAL_TITLE_LIMIT: usize = 45;

pub fn run(
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
    index: Option<usize>,
) -> Result<CreateInteractionResponse<'static>> {
    Ok(CreateInteractionResponse::Message(create_message(
        db, user_id, index,
    )))
}

pub fn update(
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
    index: Option<usize>,
) -> Result<CreateInteractionResponse<'static>> {
    Ok(CreateInteractionResponse::UpdateMessage(create_message(
        db, user_id, index,
    )))
}

fn create_message(
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
    index: Option<usize>,
) -> CreateInteractionResponseMessage<'static> {
    let characters = {
        let db = db.read();

        db.collections
            .get(&user_id)
            .map(|collection| collection.characters.clone())
    };
    if let Some(characters) = characters {
        let last_index = characters.len().saturating_sub(1);
        let current_index = index.unwrap_or(last_index).min(last_index);

        if let Some(current) = characters.get(current_index) {
            let embed = create_character_embed(current);

            let current_rank = last_index - current_index;
            let page_start_rank = (current_rank / SELECT_PAGE_SIZE) * SELECT_PAGE_SIZE;
            let newer_page_id = page_start_rank
                .checked_sub(SELECT_PAGE_SIZE)
                .map(|rank| last_index - rank);
            let older_page_rank = page_start_rank + SELECT_PAGE_SIZE;
            let older_page_id =
                (older_page_rank <= last_index).then(|| last_index - older_page_rank);

            let actions = CreateComponent::ActionRow(CreateActionRow::Buttons(
                vec![
                    CreateButton::new(format!("gift:{},{}", user_id, current_index))
                        .label("Gift")
                        .style(ButtonStyle::Success),
                    CreateButton::new(format!("delete:{},{}", user_id, current_index))
                        .label("Delete")
                        .style(ButtonStyle::Danger),
                ]
                .into(),
            ));

            let select =
                CreateComponent::ActionRow(CreateActionRow::SelectMenu(collection_select(
                    user_id,
                    characters,
                    current_index,
                    last_index,
                    page_start_rank,
                    newer_page_id,
                    older_page_id,
                )));

            return CreateInteractionResponseMessage::new()
                .embed(embed)
                .components(vec![select, actions]);
        }
    }

    CreateInteractionResponseMessage::new()
        .embed(CreateEmbed::new().title("Your collection is empty!"))
        .components(vec![])
}

pub fn select(
    db: Arc<AtomicDatabase<Database>>,
    owner_id: u64,
    kind: &ComponentInteractionDataKind,
) -> Result<CreateInteractionResponse<'static>> {
    if let ComponentInteractionDataKind::StringSelect { values } = kind {
        if let Some(value) = values.first() {
            let index = value.strip_prefix("page:").unwrap_or(value).parse().ok();

            if let Some(index) = index {
                return update(db, owner_id, Some(index));
            }
        }
    }

    Err(Error::NotFound)
}

fn collection_select(
    user_id: u64,
    characters: Vec<Character>,
    current_index: usize,
    last_index: usize,
    page_start_rank: usize,
    newer_page_id: Option<usize>,
    older_page_id: Option<usize>,
) -> CreateSelectMenu<'static> {
    let mut options = Vec::with_capacity(SELECT_PAGE_SIZE + 2);

    if let Some(index) = newer_page_id {
        options.push(
            CreateSelectMenuOption::new("more...", format!("page:{index}"))
                .description("Load newer characters in your collection"),
        );
    }

    for rank in page_start_rank..(page_start_rank + SELECT_PAGE_SIZE) {
        if rank > last_index {
            break;
        }

        let index = last_index - rank;
        if let Some(character) = characters.get(index) {
            options.push(
                CreateSelectMenuOption::new(
                    format!("{}. {}", index + 1, character.name),
                    index.to_string(),
                )
                .description(format!("Goon Credits: {}", character.goon_credits))
                .default_selection(index == current_index),
            );
        }
    }

    if let Some(index) = older_page_id {
        options.push(
            CreateSelectMenuOption::new("more...", format!("page:{index}"))
                .description("Load older characters in your collection"),
        );
    }

    CreateSelectMenu::new(
        format!("select:{user_id}"),
        CreateSelectMenuKind::String {
            options: options.into(),
        },
    )
    .placeholder("Select a character")
    .min_values(1)
    .max_values(1)
}

pub fn delete(
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
    owner_id: u64,
    index: usize,
) -> Result<CreateInteractionResponse<'static>> {
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

pub fn gift_modal(
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
    owner_id: u64,
    index: usize,
) -> Result<CreateInteractionResponse<'static>> {
    if user_id == owner_id {
        let character = {
            let db = db.read();

            db.collections
                .get(&owner_id)
                .and_then(|collection| collection.characters.get(index))
                .cloned()
        };
        if let Some(character) = character {
            Ok(CreateInteractionResponse::Modal(
                CreateModal::new(
                    format!("gift-recipient:{owner_id},{index}"),
                    truncate_chars(&format!("Gift: {}", character.name), MODAL_TITLE_LIMIT),
                )
                .components(vec![CreateModalComponent::Label(
                    CreateLabel::select_menu(
                        "Recipient",
                        CreateSelectMenu::new(
                            "gift-recipient",
                            CreateSelectMenuKind::User {
                                default_users: None,
                            },
                        )
                        .placeholder("Choose...")
                        .min_values(1)
                        .max_values(1)
                        .required(true),
                    ),
                )]),
            ))
        } else {
            Err(Error::NotFound)
        }
    } else {
        Err(Error::Unauthorized)
    }
}

pub fn gift(
    db: Arc<AtomicDatabase<Database>>,
    owner_id: u64,
    index: usize,
    resolved: &CommandDataResolved,
) -> Result<CreateInteractionResponse<'static>> {
    let recipient_id = resolved
        .users
        .iter()
        .next()
        .map(|user| user.id.get())
        .ok_or(Error::NotFound)?;

    let mut db = db.write();
    let character = db.collections.get_mut(&owner_id).and_then(|collection| {
        collection.characters.get(index)?;
        Some(collection.characters.remove(index))
    });

    if let Some(character) = character {
        if let Some(recipient_collection) = db.collections.get_mut(&recipient_id) {
            recipient_collection.characters.push(character);
        } else {
            db.collections.add(Collection {
                user_id: recipient_id,
                characters: vec![character],
            });
        }

        return Ok(CreateInteractionResponse::UpdateMessage(
            CreateInteractionResponseMessage::new()
                .content(format!("Gifted the following to <@{recipient_id}>:"))
                .components(vec![]),
        ));
    }

    Err(Error::NotFound)
}

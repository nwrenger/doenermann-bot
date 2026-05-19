use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};
use serenity::builder::CreateEmbed;

use crate::db::Database;
use crate::error::{Error, Result};
use crate::util::ResponseContent;

pub fn run(
    options: &[ResolvedOption],
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
) -> Result<ResponseContent> {
    let user_option = &options[0];

    if let ResolvedValue::User(user, _) = user_option.value {
        // Check if the current user deletes themselves
        // Otherwise check if the current user is an admin
        // If not return an unauthorized error
        let selected_id = user.id.get();
        if selected_id != user_id {
            let found = {
                let db = db.read();
                db.config
                    .server
                    .admins
                    .iter()
                    .any(|u| user_id.to_string() == *u)
            };
            if !found {
                return Err(Error::Unauthorized);
            }
        }

        let out = db.write().birthdays.delete(&selected_id);

        if out.is_some() {
            Ok(ResponseContent::new_only_embed(
                CreateEmbed::default().title(format!(
                    "The birthday of {} was successfully deleted!",
                    user.name
                )),
            ))
        } else {
            Ok(ResponseContent::new_only_embed(
                CreateEmbed::default().title(format!(
                    "The selected user {} is not inside the birthdays list!",
                    user.name
                )),
            ))
        }
    } else {
        Err(Error::OptionResolve)
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("delete_birthday")
        .description("Delete a saved birthday")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::User,
                "user",
                "Select yourself, or another user if you are an admin",
            )
            .required(true),
        )
}

use std::sync::Arc;

use light_magic::atomic::AtomicDatabase;
use serenity::all::{CreateInteractionResponseMessage, ResolvedOption, ResolvedValue};
use serenity::builder::CreateEmbed;

use crate::db::Database;
use crate::error::{Error, Result};

pub fn run(
    options: &[ResolvedOption],
    db: Arc<AtomicDatabase<Database>>,
    user_id: u64,
    admins: &[String],
) -> Result<CreateInteractionResponseMessage> {
    let user_option = options.first().ok_or(Error::OptionResolve)?;

    if let ResolvedValue::User(user, _) = user_option.value {
        // Check if the current user deletes themselves
        // Otherwise check if the current user is an admin
        // If not return an unauthorized error
        let selected_id = user.id.get();
        if selected_id != user_id {
            let user_id = user_id.to_string();
            let found = admins.iter().any(|admin| admin == &user_id);
            if !found {
                return Err(Error::Unauthorized);
            }
        }

        let out = db.write().birthdays.delete(&selected_id);

        if out.is_some() {
            Ok(
                CreateInteractionResponseMessage::new().embed(CreateEmbed::default().title(
                    format!("The birthday of {} was successfully deleted!", user.name),
                )),
            )
        } else {
            Ok(
                CreateInteractionResponseMessage::new().embed(CreateEmbed::default().title(
                    format!(
                        "The selected user {} is not inside the birthdays list!",
                        user.name
                    ),
                )),
            )
        }
    } else {
        Err(Error::OptionResolve)
    }
}

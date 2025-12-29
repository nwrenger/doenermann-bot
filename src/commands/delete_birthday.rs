use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, ResolvedOption, ResolvedValue,
};
use serenity::builder::CreateEmbed;

use crate::commands::{load_birthdays, save_birthdays};
use crate::config::Config;
use crate::error::{Error, Result};
use crate::ResponseContent;

pub fn run(
    options: &[ResolvedOption],
    config: &Config,
    current_user: u64,
) -> Result<ResponseContent> {
    let user_option = &options[0];

    if let ResolvedValue::User(user, _) = user_option.value {
        // Check if the current user deletes themselves
        // Otherwise check if the current user is an admin
        // If not return an unauthorized error
        let selected = user.id.get();
        if selected != current_user {
            let found = config
                .server
                .admins
                .iter()
                .any(|u| current_user.to_string() == *u);
            if !found {
                return Err(Error::Unauthorized);
            }
        }

        let mut rows = load_birthdays(&config.paths.birthdays)?;
        let original_len = rows.len();
        rows.retain(|e| e.user != Into::<u64>::into(user.id));

        save_birthdays(&config.paths.birthdays, &rows)?;

        if rows.len() != original_len {
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

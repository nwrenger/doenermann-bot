use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, Permissions, ResolvedOption,
    ResolvedValue,
};
use serenity::builder::CreateEmbed;

use crate::commands::{load_birthdays, save_birthdays};
use crate::error::{Error, Result};
use crate::ResponseContent;

pub fn run(options: &[ResolvedOption]) -> Result<ResponseContent> {
    let user_option = &options[0];

    if let ResolvedValue::User(user, _) = user_option.value {
        let mut rows = load_birthdays()?;
        let original_len = rows.len();
        rows.retain(|e| e.user != Into::<u64>::into(user.id));

        save_birthdays(&rows)?;

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
        .description("Delete a specific birthday. Please note: Only admins are able to do that!")
        .default_member_permissions(Permissions::CREATE_EVENTS)
        .add_option(
            CreateCommandOption::new(CommandOptionType::User, "user", "The selected user")
                .required(true),
        )
}

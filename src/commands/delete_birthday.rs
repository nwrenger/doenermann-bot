use serenity::all::{
    CommandOptionType, CreateCommand, CreateCommandOption, Permissions, ResolvedOption,
    ResolvedValue,
};
use serenity::builder::CreateEmbed;

use crate::commands::{load_birthdays, save_birthdays};
use crate::error::Error;
use crate::ResponseContent;

pub fn run(options: &[ResolvedOption]) -> ResponseContent {
    let user_option = &options[0];

    if let ResolvedValue::User(user, _) = user_option.value {
        match load_birthdays() {
            Ok(mut rows) => {
                rows.retain(|e| e.user != Into::<u64>::into(user.id));

                if let Err(e) = save_birthdays(&rows) {
                    return e.error_message();
                }

                ResponseContent::new_only_embed(CreateEmbed::default().title(format!(
                    "The birthday of {} was successfully deleted!",
                    user.name
                )))
            }
            Err(e) => e.error_message(),
        }
    } else {
        Error::OptionResolve.error_message()
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

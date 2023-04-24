use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> (String, CreateEmbed) {
    let mut embed = CreateEmbed::default();
    embed.title("Next Birthdays:");
    for i in 0..10 {
        embed.field((i + 1).to_string(), "Test Text", false);
    }
    ("".to_string(),embed,)
}

pub fn _register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("next_birthdays").description("The next 10 Upcomming Birthdays")
}

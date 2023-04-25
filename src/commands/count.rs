use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(
    _options: &[CommandDataOption],
    count: i32,
    count_list: &mut [String],
) -> (String, CreateEmbed) {
    let mut embed = CreateEmbed::default();
    embed.title(format!("Already recorded messages: {}\nList of already recorded messages:", count));
    for i in count_list {
        embed.field("", i, false);
    }
    ("".to_string(), embed)
}

pub fn _register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("count")
        .description("Gives the Count of the already Recorded Messages after last Start")
}

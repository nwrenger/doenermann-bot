use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption], count: i32, count_list: &mut [String]) -> (String, CreateEmbed) {
    let title = "Already recorded messages: ".to_string() + &count.to_string() + "\nList of already recorded messages:";
    let mut embed: CreateEmbed = CreateEmbed::default();
    embed.title(title);
    for i in 0..count_list.len() {
        embed.field("", count_list[i].clone(), false);
    }
    ("".to_string(), embed)
}

pub fn _register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("count").description("Gives the Count of the already Recorded Messages after last Start")
}

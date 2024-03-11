use serenity::{
    all::{CreateCommand, ResolvedOption},
    builder::CreateEmbed,
};

use crate::ResponseContent;

pub fn run(_options: &[ResolvedOption], count: &i32, count_list: &[String]) -> ResponseContent {
    let title = if count_list.is_empty() {
        "No messages have been recorded after last startup!".to_string()
    } else {
        format!(
            "Already recorded messages: {}\nList of already recorded messages:",
            count
        )
    };
    let mut embed = CreateEmbed::default().title(title);
    for (i, item) in count_list.iter().enumerate() {
        if i < 25 {
            embed = embed.field("", item, false);
        } else {
            embed = embed.field("", "...", false);
        }
    }
    ResponseContent {
        text: "".to_string(),
        embed,
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("count")
        .description("Gives the Count of the already Recorded Messages after last Start")
}

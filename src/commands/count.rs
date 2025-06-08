use serenity::{
    all::{CreateCommand, ResolvedOption},
    builder::CreateEmbed,
};

use crate::error::Result;
use crate::ResponseContent;

pub fn run(
    _options: &[ResolvedOption],
    count: &i32,
    count_list: &[String],
) -> Result<ResponseContent> {
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
        // We have to get the first 24 items
        if i < 24 {
            embed = embed.field("", item, false);
        } else {
            embed = embed.field("", "...", false);
        }
    }

    Ok(ResponseContent::new_only_embed(embed))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("count")
        .description("Gives the Count of the already Recorded Messages after last Start")
}

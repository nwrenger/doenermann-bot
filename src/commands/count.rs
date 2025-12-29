use serenity::{
    all::{CreateCommand, ResolvedOption},
    builder::CreateEmbed,
};

use crate::ResponseContent;
use crate::{error::Result, Counter};

pub fn run(_options: &[ResolvedOption], counter: &Counter) -> Result<ResponseContent> {
    let title = if counter.list.is_empty() {
        "No messages have been recorded after last startup!".to_string()
    } else {
        format!(
            "Already recorded messages: {}\nList of already recorded messages:",
            counter.count
        )
    };
    let mut embed = CreateEmbed::default().title(title);
    for (i, item) in counter.list.iter().enumerate() {
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
        .description("Show which and how many messages were recorded since the last start")
}

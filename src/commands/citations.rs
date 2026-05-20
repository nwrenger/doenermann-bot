use std::sync::Arc;

use chrono::Local;
use light_magic::atomic::AtomicDatabase;
use serenity::{
    all::{CreateCommand, ResolvedOption},
    builder::CreateEmbed,
};

use crate::error::Result;
use crate::{db::Database, util::ResponseContent};

const MAX_CITATION_FIELDS: usize = 25;
const MAX_EMBED_CHARS: usize = 6000;
const MAX_FIELD_NAME_CHARS: usize = 256;
const MAX_FIELD_VALUE_CHARS: usize = 1024;
const TRUNCATION_SUFFIX: &str = "...";
const EMPTY_FIELD_VALUE: &str = "\u{200B}";

pub fn run(
    _options: &[ResolvedOption],
    db: Arc<AtomicDatabase<Database>>,
    timestamp_format: &str,
) -> Result<ResponseContent> {
    let citations = {
        let db = db.read();
        db.citations
            .values()
            .rev()
            .take(MAX_CITATION_FIELDS)
            .cloned()
            .collect::<Vec<_>>()
    };

    let title = if citations.is_empty() {
        "No citations have been recorded!".to_string()
    } else {
        format!("Recorded Citations: {}", citations.len())
    };

    let mut remaining_chars = MAX_EMBED_CHARS.saturating_sub(embed_title_len(&title));
    let mut embed = CreateEmbed::default().title(title);

    for message in citations {
        if remaining_chars < 2 {
            break;
        }

        let field_name = truncate_text(
            message
                .utc_timestamp
                .with_timezone(&Local)
                .format(timestamp_format)
                .to_string(),
            MAX_FIELD_NAME_CHARS.min(remaining_chars - 1),
        );
        remaining_chars = remaining_chars.saturating_sub(field_name.chars().count());

        let max_value_chars = MAX_FIELD_VALUE_CHARS.min(remaining_chars);
        let field_value = if max_value_chars == 0 {
            TRUNCATION_SUFFIX.to_string()
        } else if message.content.trim().is_empty() {
            truncate_text(
                format!("<@{}>\n{}", message.user_id, EMPTY_FIELD_VALUE),
                max_value_chars,
            )
        } else {
            truncate_text(
                format!("<@{}>\n{}", message.user_id, message.content),
                max_value_chars,
            )
        };
        remaining_chars = remaining_chars.saturating_sub(field_value.chars().count());

        embed = embed.field(field_name, field_value, false);
    }

    Ok(ResponseContent::new_only_embed(embed))
}

fn embed_title_len(title: &str) -> usize {
    title.chars().count()
}

fn truncate_text(value: String, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value;
    }

    if max_chars <= TRUNCATION_SUFFIX.len() {
        return TRUNCATION_SUFFIX
            .chars()
            .take(max_chars)
            .collect::<String>();
    }

    value
        .chars()
        .take(max_chars - TRUNCATION_SUFFIX.len())
        .collect::<String>()
        + TRUNCATION_SUFFIX
}

pub fn register() -> CreateCommand {
    CreateCommand::new("citations").description("Show which and how many citations were recorded")
}

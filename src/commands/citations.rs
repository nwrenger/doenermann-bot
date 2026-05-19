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
) -> Result<ResponseContent> {
    let (timestamp_format, mut citations) = {
        let db = db.read();
        (
            db.config.bot.timestamp_format.clone(),
            db.citations.values().cloned().collect::<Vec<_>>(),
        )
    };

    citations.sort_by(|a, b| {
        b.utc_timestamp
            .cmp(&a.utc_timestamp)
            .then_with(|| b.id.cmp(&a.id))
    });

    let title = if citations.is_empty() {
        "No citations have been recorded!".to_string()
    } else {
        format!("Recorded Citations: {}", citations.len())
    };

    let mut remaining_chars = MAX_EMBED_CHARS.saturating_sub(embed_title_len(&title));
    let mut embed = CreateEmbed::default().title(title);

    for message in citations.iter().take(MAX_CITATION_FIELDS) {
        if remaining_chars < 2 {
            break;
        }

        let field_name = truncate_text(
            format!(
                "<@{}> [{}]",
                message.user.id,
                message
                    .utc_timestamp
                    .with_timezone(&Local)
                    .format(&timestamp_format)
            ),
            MAX_FIELD_NAME_CHARS.min(remaining_chars - 1),
        );
        remaining_chars = remaining_chars.saturating_sub(field_name.chars().count());

        let max_value_chars = MAX_FIELD_VALUE_CHARS.min(remaining_chars);
        let field_value = if max_value_chars == 0 {
            TRUNCATION_SUFFIX.to_string()
        } else if message.content.trim().is_empty() {
            EMPTY_FIELD_VALUE.to_string()
        } else {
            truncate_text(message.content.clone(), max_value_chars)
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

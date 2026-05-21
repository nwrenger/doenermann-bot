use serenity::all::Embed;

pub const EMPTY_FIELD_VALUE: &str = "\u{200B}";

pub fn embeds_to_string(embeds: &[Embed]) -> String {
    embeds
        .iter()
        .filter_map(embed_to_string)
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn embed_to_string(embed: &Embed) -> Option<String> {
    let mut parts = Vec::new();

    if let Some(author) = &embed.author {
        push_non_empty(&mut parts, &author.name);
    }

    if let Some(title) = &embed.title {
        push_non_empty(&mut parts, title);
    }

    if let Some(description) = &embed.description {
        push_non_empty(&mut parts, description);
    }

    embed.fields.iter().for_each(|field| {
        let name = field.name.trim();
        let value = field.value.trim();

        match (name.is_empty(), value.is_empty()) {
            (false, false) => parts.push(format!("{name}: {value}")),
            (false, true) => parts.push(name.to_string()),
            (true, false) => parts.push(value.to_string()),
            (true, true) => {}
        }
    });

    if let Some(footer) = &embed.footer {
        push_non_empty(&mut parts, &footer.text);
    }

    if let Some(url) = &embed.url {
        push_non_empty(&mut parts, url);
    }

    (!parts.is_empty()).then(|| parts.join("\n"))
}

fn push_non_empty(parts: &mut Vec<String>, value: &str) {
    let value = value.trim();

    if !value.is_empty() {
        parts.push(value.to_string());
    }
}

pub fn color_goon_credits(goon_credits: u32) -> Option<u32> {
    match goon_credits {
        0..=199 => None,
        200..=499 => Some(0x8E8E93),       // Common
        500..=999 => Some(0x2ECC71),       // Uncommon
        1_000..=2_499 => Some(0x3498DB),   // Rare
        2_500..=4_999 => Some(0x9B59B6),   // Epic
        5_000..=9_999 => Some(0xF1C40F),   // Legendary
        10_000..=19_999 => Some(0xE67E22), // Mythic
        _ => Some(0xE91E63),               // Icon
    }
}

pub fn from_collection_payload(payload: &str) -> Option<(u64, usize)> {
    let (owner_id, index) = payload.split_once(',')?;
    Some((owner_id.parse().ok()?, index.parse().ok()?))
}

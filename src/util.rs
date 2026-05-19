use serenity::all::{CreateEmbed, Embed};

pub struct ResponseContent {
    pub text: String,
    pub embed: CreateEmbed,
}

impl ResponseContent {
    pub fn new(text: String, embed: CreateEmbed) -> Self {
        Self { text, embed }
    }

    pub fn new_only_embed(embed: CreateEmbed) -> Self {
        Self {
            text: String::new(),
            embed,
        }
    }
}

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

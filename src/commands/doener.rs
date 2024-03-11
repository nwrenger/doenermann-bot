use serenity::all::{CreateCommand, ResolvedOption};
use serenity::builder::CreateEmbed;

use crate::ResponseContent;

pub fn run(_options: &[ResolvedOption]) -> ResponseContent {
    ResponseContent {
        text: "Ne diggi, denkste ich habe das Geld dafür? Aber hier das sollte dir helfen:".to_string(),
        embed: CreateEmbed::default().title("Döner bestellen in 30159 Hannover | Lieferando.de").url("https://www.lieferando.de/lieferservice/doener/hannover-30159").description("Bestelle Döner in 30159 Hannover online über Lieferando.de. Food Tracker® und verschiedene Bezahlmethoden. Genieße Deine Döner Lieferung!")
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("döner").description("Döner bestellen?")
}

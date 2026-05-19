use serenity::all::{CreateCommand, ResolvedOption};
use serenity::builder::CreateEmbed;

use crate::error::Result;
use crate::util::ResponseContent;

pub fn run(_options: &[ResolvedOption]) -> Result<ResponseContent> {
    Ok(ResponseContent::new(
        String::from("Ne diggi, denkste ich habe das Geld dafür? Aber hier das sollte dir helfen:"),
        CreateEmbed::default().title("Döner bestellen in 30159 Hannover | Lieferando.de").url("https://www.lieferando.de/lieferservice/doener/hannover-30159").description("Bestelle Döner in 30159 Hannover online über Lieferando.de. Food Tracker® und verschiedene Bezahlmethoden. Genieße Deine Döner Lieferung!")
    ))
}

pub fn register() -> CreateCommand {
    CreateCommand::new("döner").description("Döner bestellen?")
}

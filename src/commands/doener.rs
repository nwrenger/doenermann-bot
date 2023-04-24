use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> (String, CreateEmbed) {
    let mut embed = CreateEmbed::default();
    embed.title("Döner bestellen in 30159 Hannover | Lieferando.de");
    embed.url("https://www.lieferando.de/lieferservice/doener/hannover-30159");
    embed.description("Bestelle Döner in 30159 Hannover online über Lieferando.de. Food Tracker® und verschiedene Bezahlmethoden. Genieße Deine Döner Lieferung!");
    (
        "Ne diggi, denkste ich habe das Geld dafür? Aber hier das sollte dir helfen:".to_string(),
        embed,
    )
}

pub fn _register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("döner").description("Döner bestellen?")
}

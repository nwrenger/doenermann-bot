mod api;
mod commands;
mod config;
mod db;
mod error;
mod util;

use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use light_magic::atomic::{AtomicDatabase, DataStore};
use serenity::async_trait;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::Config;
use crate::db::{Character, Database};
use crate::error::Error;
use crate::util::{embeds_to_string, from_collection_payload};

struct SharedData {
    config: Arc<Config>,
    db: Arc<AtomicDatabase<Database>>,
}

impl SharedData {
    fn new(config: Arc<Config>, db: Arc<AtomicDatabase<Database>>) -> Self {
        Self { config, db }
    }
}

struct Handler;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_PATH: &str = "config.toml";

#[async_trait]
impl EventHandler for Handler {
    async fn dispatch(&self, ctx: &Context, event: &FullEvent) {
        let shared_data = ctx.data::<SharedData>();
        let config = shared_data.config.clone();
        let db = shared_data.db.clone();
        match event {
            // Add a role specified in the config on server join
            FullEvent::GuildMemberAddition { new_member, .. } => {
                let role_id: u64 = config
                    .server
                    .role_on_join
                    .parse()
                    .expect("Role on join must be a valid integer string");

                new_member
                    .add_role(&ctx.http, role_id.into(), None)
                    .await
                    .unwrap()
            }
            // Copies text messages of a citation channel to the database.
            FullEvent::Message { new_message, .. } => {
                let citations_channel: u64 = config
                    .server
                    .citations_channel
                    .parse()
                    .expect("Copied Channel must be a valid integer string");

                if new_message.channel_id == GenericChannelId::new(citations_channel) {
                    db.write().citations.add(db::Message::new(
                        new_message.id.into(),
                        new_message.author.id.into(),
                        format!(
                            "{}{}",
                            new_message.content,
                            embeds_to_string(&new_message.embeds)
                                .lines()
                                .map(|line| format!("\n{line}"))
                                .collect::<String>()
                        ),
                        new_message.timestamp.to_utc(),
                    ));
                };
            }
            // Commands handler
            FullEvent::InteractionCreate { interaction, .. } => match interaction {
                Interaction::Command(command) => {
                    let content = match command.data.name.as_str() {
                        "citations" => commands::citations::run(
                            &command.data.options(),
                            db,
                            &shared_data.config.bot.timestamp_format,
                        ),
                        "birthday" => commands::birthday::run(
                            &command.data.options(),
                            db,
                            command.user.id.into(),
                            &shared_data.config.bot.date_format,
                            &shared_data.config.server.admins,
                        ),
                        "döner" => commands::doener::run(&command.data.options()),
                        "waifu" => {
                            commands::waifu::run(
                                &command.data.options(),
                                db,
                                command.user.id.into(),
                            )
                            .await
                        }
                        _ => Err(Error::NotFound),
                    };
                    match content {
                        Ok(content) => {
                            if let Err(err) = command.create_response(&ctx.http, content).await {
                                eprintln!("Failed to create command response: {err}");
                            }
                        }
                        Err(e) => {
                            if let Err(err) =
                                command.create_response(&ctx.http, e.error_response()).await
                            {
                                eprintln!("Failed to create command error response: {err}");
                            }
                        }
                    }
                }
                Interaction::Component(component) => {
                    let custom_id = &component.data.custom_id;
                    let (id, payload) = custom_id.split_once(':').unwrap_or_default();

                    let content = match id {
                        "claim" if let Some(character) = Character::from_payload(payload) => {
                            commands::waifu::roll::claim(
                                db,
                                component.user.id.into(),
                                &component.user.name,
                                character,
                            )
                        }
                        "delete"
                            if let Some((owner_id, index)) = from_collection_payload(payload) =>
                        {
                            commands::waifu::collection::delete(
                                db,
                                component.user.id.into(),
                                owner_id,
                                index,
                            )
                        }
                        "gift"
                            if let Some((owner_id, index)) = from_collection_payload(payload) =>
                        {
                            commands::waifu::collection::gift_modal(
                                db,
                                component.user.id.into(),
                                owner_id,
                                index,
                            )
                        }
                        "select" if let Ok(owner_id) = payload.parse() => {
                            commands::waifu::collection::select(db, owner_id, &component.data.kind)
                        }
                        _ => Err(Error::NotFound),
                    };

                    match content {
                        Ok(content) => {
                            if let Err(err) = component.create_response(&ctx.http, content).await {
                                eprintln!("Failed to create component update response: {err}");
                            }
                        }
                        Err(e) => {
                            if let Err(response_err) = component
                                .create_response(&ctx.http, e.error_response())
                                .await
                            {
                                eprintln!(
                                    "Failed to create component error response: {response_err}"
                                );
                            }
                        }
                    }
                }
                Interaction::Modal(modal) => {
                    let custom_id = &modal.data.custom_id;
                    let (id, payload) = custom_id.split_once(':').unwrap_or_default();

                    let content = match id {
                        "gift-recipient"
                            if let Some((owner_id, index)) = from_collection_payload(payload) =>
                        {
                            commands::waifu::collection::gift(
                                db,
                                owner_id,
                                index,
                                &modal.data.resolved,
                            )
                        }
                        _ => Err(Error::NotFound),
                    };

                    match content {
                        Ok(content) => {
                            if let Err(err) = modal.create_response(&ctx.http, content).await {
                                eprintln!("Failed to create modal response: {err}");
                            }
                        }
                        Err(e) => {
                            if let Err(response_err) =
                                modal.create_response(&ctx.http, e.error_response()).await
                            {
                                eprintln!("Failed to create modal error response: {response_err}");
                            }
                        }
                    }
                }
                _ => return,
            },
            // Setting stuff up on start
            FullEvent::Ready { data_about_bot, .. } => {
                let date_format = config.bot.date_format.clone();
                let parsed_guild: u64 = config
                    .server
                    .guild
                    .parse()
                    .expect("Guild ID must be a valid integer string");

                println!(
                    "'{}' connecting with Guild (id): {:?}",
                    data_about_bot.user.name, &parsed_guild
                );

                GuildId::new(parsed_guild)
                    .set_commands(
                        &ctx.http,
                        &[
                            commands::birthday::register(date_format),
                            commands::citations::register(),
                            commands::doener::register(),
                            commands::waifu::register(),
                        ],
                    )
                    .await
                    .unwrap();
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Starting Bot (v{})", PKG_VERSION);

    let config_path = PathBuf::from(CONFIG_PATH);
    let config = Config::read_or_create(config_path).expect("Expected a valid config!");
    let db = Database::open(&config.paths.database);
    let token = Token::from_str(&config.bot.token).expect("Expected a valid token in the config!");
    let data = SharedData::new(Arc::new(config), Arc::new(db));

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Build our client
    let mut client = Client::builder(token, intents)
        .data::<SharedData>(Arc::new(data))
        .event_handler(Arc::new(Handler))
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

mod commands;
mod config;
mod db;
mod error;
mod util;

use std::path::PathBuf;
use std::sync::Arc;

use light_magic::atomic::DataStore;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::Config;
use crate::db::Database;
use crate::error::Error;
use crate::util::embeds_to_string;

struct Handler;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_PATH: &str = "config.toml";

#[async_trait]
impl EventHandler for Handler {
    // Add a role specified in the config on server join
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let data = ctx.data.read().await;
        let config = data.get::<Config>().expect("Expected a Config").clone();

        let role_id: u64 = config
            .server
            .role_on_join
            .parse()
            .expect("Role on join must be a valid integer string");

        new_member.add_role(&ctx.http, role_id).await.unwrap()
    }

    // Copies text messages of a citation channel to the database.
    async fn message(&self, ctx: Context, msg: Message) {
        let (config, db) = {
            let data = ctx.data.read().await;
            (
                data.get::<Config>().expect("Expected a Config").clone(),
                data.get::<Database>().expect("Expected a Database").clone(),
            )
        };

        let citations_channel: u64 = config
            .server
            .citations_channel
            .parse()
            .expect("Copied Channel must be a valid integer string");

        if msg.channel_id == ChannelId::new(citations_channel) {
            db.write().citations.add(db::Message::new(
                msg.id.into(),
                msg.author.id.into(),
                format!(
                    "{}{}",
                    msg.content,
                    embeds_to_string(&msg.embeds)
                        .lines()
                        .map(|line| format!("\n{line}"))
                        .collect::<String>()
                ),
                msg.timestamp.to_utc(),
            ));
        }
    }

    // Commands handler
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let (config, db) = {
            let data = ctx.data.read().await;
            (
                data.get::<Config>().expect("Expected a Config").clone(),
                data.get::<Database>().expect("Expected a Database").clone(),
            )
        };

        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "citations" => commands::citations::run(
                    &command.data.options(),
                    db,
                    &config.bot.timestamp_format,
                ),
                "birthday" => commands::birthday::run(
                    &command.data.options(),
                    db,
                    command.user.id.into(),
                    &config.bot.date_format,
                    &config.server.admins,
                ),
                "döner" => commands::doener::run(&command.data.options()),
                _ => Err(Error::CommandNotFound),
            };
            match content {
                Ok(content) => {
                    if let Err(why) = command
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .content(content.text)
                                    .add_embed(content.embed),
                            ),
                        )
                        .await
                    {
                        command
                            .create_response(
                                &ctx.http,
                                CreateInteractionResponse::Message(
                                    CreateInteractionResponseMessage::new().add_embed(
                                        CreateEmbed::default()
                                            .color(Colour::RED)
                                            .title(format!("An unknwon Error occurred: {why}!")),
                                    ),
                                ),
                            )
                            .await
                            .unwrap_or_default();
                    };
                }
                Err(e) => {
                    let error_message = e.error_message();
                    command
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .add_embed(error_message.embed),
                            ),
                        )
                        .await
                        .unwrap_or_default();
                }
            }
        }
    }

    // Setting stuff up on start
    async fn ready(&self, ctx: Context, ready: Ready) {
        let data = ctx.data.read().await;
        let config = data.get::<Config>().expect("Expected a Config");
        let date_format = config.bot.date_format.clone();
        let parsed_guild: u64 = config
            .server
            .guild
            .parse()
            .expect("Guild ID must be a valid integer string");

        println!(
            "'{}' connecting with Guild (id): {:?}",
            ready.user.name, &parsed_guild
        );

        GuildId::new(parsed_guild)
            .set_commands(
                &ctx.http,
                vec![
                    commands::birthday::register(date_format),
                    commands::citations::register(),
                    commands::doener::register(),
                ],
            )
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    println!("Starting Bot (v{})", PKG_VERSION);

    let config_path = PathBuf::from(CONFIG_PATH);
    let config = Config::read_or_create(config_path).expect("Expected a valid config!");
    let db = Database::open(&config.paths.database);

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Build our client
    let mut client = Client::builder(&config.bot.token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Setting up config && database
    {
        let mut data = client.data.write().await;
        data.insert::<Config>(Arc::new(config));
        data.insert::<Database>(Arc::new(db));
    }

    // Finally, start a single shard, and start listening to events
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

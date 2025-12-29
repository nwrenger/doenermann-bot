mod commands;
mod config;
mod error;

use chrono::offset::Local;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::vec;

use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::config::Config;
use crate::error::Error;

struct ResponseContent {
    text: String,
    embed: CreateEmbed,
}

impl ResponseContent {
    fn new(text: String, embed: CreateEmbed) -> Self {
        Self { text, embed }
    }

    fn new_only_embed(embed: CreateEmbed) -> Self {
        Self {
            text: String::new(),
            embed,
        }
    }
}

#[derive(Default)]
struct Counter {
    count: i32,
    list: Vec<String>,
}

impl TypeMapKey for Counter {
    type Value = Counter;
}

struct Handler;

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CONFIG_PATH: &str = "config.toml";

#[async_trait]
impl EventHandler for Handler {
    // Add a role specified in the config on server join
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let data = ctx.data.read().await;
        let config = data.get::<Config>().expect("Expected a Config");

        let role_id: u64 = config
            .server
            .role_on_join
            .parse()
            .expect("Role on join must be a valid integer string!");

        new_member.add_role(&ctx.http, role_id).await.unwrap()
    }

    // Copies text messages of a certain channel(specified in the config) in a file.
    // It also adds and increments the Counter used in the count command.
    async fn message(&self, ctx: Context, msg: Message) {
        let mut data = ctx.data.write().await;
        let config = data.get::<Config>().expect("Expected a Config");

        let copied_channel: u64 = config
            .server
            .copy_channel
            .parse()
            .expect("Copied Channel must be a valid integer string!");

        let mut messages_file = OpenOptions::new()
            .append(true)
            .open(&config.paths.messages)
            .expect(&format!("Couldn't open {}", &config.paths.messages));

        if msg.channel_id == ChannelId::new(copied_channel) {
            let user_message = format!(
                "{}: {} | {}\n",
                msg.author.name,
                msg.content.replace('\n', " - "),
                msg.timestamp
            );

            messages_file
                .write_all(user_message.as_bytes())
                .expect("Couldn't write to file");

            // update counter
            if let Some(counter) = data.get_mut::<Counter>() {
                counter.count += 1;
                counter.list.push(user_message);
            }
        }
    }

    // Commands handler
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "count" => {
                    let data = ctx.data.read().await;
                    commands::count::run(
                        &command.data.options(),
                        data.get::<Counter>().unwrap_or(&Counter::default()),
                    )
                }
                "delete_birthday" => {
                    let data = ctx.data.read().await;
                    commands::delete_birthday::run(
                        &command.data.options(),
                        data.get::<Config>().unwrap_or(&Config::new()),
                        command.user.id.into(),
                    )
                }
                "döner" => commands::doener::run(&command.data.options()),
                "next_birthdays" => {
                    let data = ctx.data.read().await;
                    commands::next_birthdays::run(data.get::<Config>().unwrap_or(&Config::new()))
                }
                "set_birthday" => {
                    let data = ctx.data.read().await;
                    commands::set_birthday::run(
                        &command.data.options(),
                        data.get::<Config>().unwrap_or(&Config::new()),
                        command.user.id.into(),
                    )
                }
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
        println!(
            "{} is connected with Servers {:?}!",
            ready.user.name,
            ready
                .guilds
                .iter()
                .map(|f| f.id.to_string())
                .collect::<Vec<String>>()
        );

        let copy_message = format!("[Info] Begin Copying on {}\n", Local::now().date_naive());
        let data = ctx.data.read().await;
        let config = data.get::<Config>().expect("Expected a Config");

        let mut messages_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&config.paths.messages)
            .expect(&format!("Couldn't open {}", &config.paths.messages));

        messages_file
            .write_all(copy_message.as_bytes())
            .expect("Couldn't write to file");

        let _birthday_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&config.paths.birthdays)
            .expect(&format!("Couldn't open {}", &config.paths.birthdays));

        for UnavailableGuild { id, .. } in ready.guilds {
            id.set_commands(
                &ctx.http,
                vec![
                    commands::count::register(),
                    commands::delete_birthday::register(),
                    commands::doener::register(),
                    commands::next_birthdays::register(),
                    commands::set_birthday::register(&config),
                ],
            )
            .await
            .unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    println!("Starting bot on Version {}...", PKG_VERSION);

    let config_path = PathBuf::from(CONFIG_PATH);
    let config = Config::read_or_create(config_path).expect("Expected a valid config!");

    let intents = GatewayIntents::all();

    // Build our client
    let mut client = Client::builder(&config.bot.token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Setting counter && config
    {
        let mut data = client.data.write().await;
        data.insert::<Counter>(Counter::default());
        data.insert::<Config>(config);
    }

    // Finally, start a single shard, and start listening to events
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

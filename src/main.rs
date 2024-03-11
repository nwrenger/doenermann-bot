mod commands;

use chrono::offset::Local;
use dotenv::dotenv;
use serenity::all::{CreateInteractionResponse, CreateInteractionResponseMessage};
use std::fs::OpenOptions;
use std::io::Write;
use std::{env, vec};

use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::*;
use serenity::prelude::*;

struct ResponseContent {
    text: String,
    embed: CreateEmbed,
}

struct Count;
struct CountList;
struct Handler;

impl TypeMapKey for Count {
    type Value = i32;
}

impl TypeMapKey for CountList {
    type Value = Vec<String>;
}

const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

#[async_trait]
impl EventHandler for Handler {
    //add a role specified in the env on server join
    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        let role_id: u64 = env::var("ROLE_ID")
            .expect("Expected ROLE_ID in environment")
            .parse()
            .expect("ROLE_ID must be an Integer!");
        new_member.add_role(&ctx.http, role_id).await.unwrap()
    }
    //Copies text messages of a certain channel(specified in the env, C_CHANNEL_ID) in a file named "citatins.txt".
    //It also adds and increments the values (COUNT,COUNT_LIST) used in the count command.
    async fn message(&self, ctx: Context, msg: Message) {
        let copied_channel: u64 = env::var("C_CHANNEL_ID")
            .expect("Expected C_CHANNEL_ID in environment")
            .parse()
            .expect("C_CHANNEL_ID must be an Integer!");

        let mut file = OpenOptions::new()
            .append(true)
            .open("citations.txt")
            .expect("Couldn't open citations.txt");

        if msg.channel_id == ChannelId::new(copied_channel) {
            let user_message = format!(
                "{}: {}\n",
                msg.author.name,
                msg.content.replace('\n', " - ")
            );
            file.write_all(user_message.as_bytes())
                .expect("Couldn't write to file");
            // update globals
            let mut data = ctx.data.write().await;
            if let Some(counter) = data.get_mut::<Count>() {
                *counter += 1;
            }
            if let Some(list) = data.get_mut::<CountList>() {
                list.push(user_message);
            }
        }
    }
    //commands handler
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "döner" => commands::doener::run(&command.data.options()),
                "count" => {
                    let data = ctx.data.read().await;
                    commands::count::run(
                        &command.data.options(),
                        data.get::<Count>().unwrap_or(&0),
                        data.get::<CountList>().unwrap_or(&Vec::new()),
                    )
                }
                "set_birthday" => {
                    commands::set_birthday::run(&command.data.options(), command.user.id.into())
                }
                "next_birthdays" => commands::next_birthdays::run(&command.data.options()),
                _ => {
                    let embed = CreateEmbed::default().title("Command not Found!");
                    ResponseContent {
                        text: "".to_string(),
                        embed,
                    }
                }
            };
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
                println!("Cannot respond to slash command: {}", why);
            };
        }
    }

    // setting stuff up on start
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let copy_message = format!("[Info] Begin Copying on {}\n", Local::now().date_naive());

        let mut file_cit = OpenOptions::new()
            .append(true)
            .create(true)
            .open("citations.txt")
            .expect("Couldn't open citations.txt");

        file_cit
            .write_all(copy_message.as_bytes())
            .expect("Couldn't write to file");

        let _file_birth = OpenOptions::new()
            .append(true)
            .create(true)
            .open("birthdays.csv")
            .expect("Couldn't open birthdays.csv");

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::doener::register(),
                    commands::count::register(),
                    commands::next_birthdays::register(),
                    commands::set_birthday::register(),
                ],
            )
            .await
            .unwrap();
    }
}

#[tokio::main]
async fn main() {
    println!("Starting bot on Version {}...", PKG_VERSION);
    // Configure the client with your Discord bot token in the .env file.
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::all();

    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // setting global vars
    {
        let mut data = client.data.write().await;
        data.insert::<Count>(0);
        data.insert::<CountList>(Vec::new());
    }

    // Finally, start a single shard, and start listening to events.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

mod commands;

use chrono::offset::Local;
use dotenv::dotenv;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;

use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::*;
use serenity::prelude::*;

struct Handler;

static mut COUNT: i32 = 0;
static mut COUNT_LIST: Vec<String> = vec![];

#[async_trait]
impl EventHandler for Handler {
    //add a role specified in the env on server join
    async fn guild_member_addition(&self, ctx: Context, mut new_member: Member) {
        let role_id: u64 = env::var("ROLE_ID")
            .expect("Expected ROLE_ID in environment")
            .parse()
            .expect("ROLE_ID must be an Integer!");
        new_member.add_role(&ctx.http, role_id).await.unwrap()
    }
    //Copies text messages of a certain channel(specified in the env, C_CHANNEL_ID) in a file named "citatins.txt".
    //It also adds and increments the values (COUNT,COUNT_LIST) used in the count command.
    async fn message(&self, _ctx: Context, msg: Message) {
        let copied_channel: u64 = env::var("C_CHANNEL_ID")
            .expect("Expected C_CHANNEL_ID in environment")
            .parse()
            .expect("C_CHANNEL_ID must be an Integer!");

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("citations.txt")
            .expect("Couldn't open citations.txt");

        if msg.channel_id.as_u64() == &copied_channel {
            let user_message = format!("{}: {}\n", msg.author.name, msg.content);
            file.write_all(user_message.as_bytes())
                .expect("Couldn't write to file");
            print!("{}", user_message);
            unsafe {
                COUNT += 1;
                COUNT_LIST.push(user_message);
            }
        }
    }
    //commands handler
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            unsafe {
                let content = match command.data.name.as_str() {
                    "döner" => commands::doener::run(&command.data.options),
                    "count" => commands::count::run(&command.data.options, COUNT, &mut COUNT_LIST),
                    "set_birthday" => {
                        commands::set_birthday::run(&command.data.options, command.user.id.into())
                    }
                    "next_birthdays" => commands::next_birthdays::run(&command.data.options),
                    _ => {
                        let mut embed = CreateEmbed::default();
                        embed.title("Command not Found!");
                        ("".to_string(), embed)
                    }
                };

                if let Err(why) = command
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| {
                                message.content(content.0).add_embed(content.1)
                            })
                    })
                    .await
                {
                    println!("Cannot respond to slash command: {}", why);
                }
            }
        }
    }

    //setting stuff up on start
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let copy_message = format!("Begin Copying on {}:\n", Local::now().time());

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("citations.txt")
            .expect("Couldn't open citations.txt");

        file.write_all(copy_message.as_bytes())
            .expect("Couldn't write to file");

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| commands::doener::_register(command));
            commands.create_application_command(|command| commands::count::_register(command));
            commands
                .create_application_command(|command| commands::set_birthday::_register(command));
            commands
                .create_application_command(|command| commands::next_birthdays::_register(command))
        })
        .await
        .unwrap();
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the .env file.
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::all();

    // Build our client.
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

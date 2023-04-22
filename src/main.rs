mod commands;

use dotenv::dotenv;
use std::env;

use serenity::async_trait;
use serenity::builder::CreateEmbed;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use serenity::model::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        let copied_channel: u64 = env::var("C_CHANNEL_ID ")
            .expect("Expected C_CHANNEL_ID in environment")
            .parse()
            .expect("C_CHANNEL_ID must be an Integer!");
        println!("{copied_channel}");
        if msg.channel_id.as_u64() == &copied_channel {
            println!("{}", msg.content);
        }
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "döner" => commands::ping::run(&command.data.options),
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

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = GuildId::set_application_commands(&_guild_id, &_ctx.http, |commands| {
            commands.create_application_command(|command| commands::ping::_register(command))
        })
        .await;

        // println!("I now have the following guild slash commands: {:#?}", commands);
        // }
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the .env file.
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // Build our client.
    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

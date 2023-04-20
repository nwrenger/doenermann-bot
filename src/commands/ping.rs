use serenity::framework::standard::*;
use serenity::model::channel;
use serenity::prelude::*;

#[macros::command]
async fn ping(context: &Context, msg: &channel::Message) -> CommandResult {
    msg.reply(context, "pong!").await?;

    Ok(())
}
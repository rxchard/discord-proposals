use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::interact::proposal;

#[command]
pub async fn post_interact(ctx: &Context, message: &Message) -> CommandResult {
    proposal::create_message_in_channel(&ctx, &message.channel_id).await?;
    message.delete(&ctx).await?;

    Ok(())
}

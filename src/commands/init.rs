use serenity::{
    client::Context,
    framework::standard::{macros::command, CommandResult},
    model::channel::Message,
};

use crate::interact::proposal;

#[command]
#[required_permissions("ADMINISTRATOR")]
pub async fn post_interact(ctx: &Context, message: &Message) -> CommandResult {
    proposal::create_message_in_channel(&ctx, &message.channel_id).await?;
    let _ = message.delete(&ctx).await;

    Ok(())
}

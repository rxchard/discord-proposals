use serenity::{client::Context, model::id::ChannelId};

use crate::data_store::DataStore;

/// Increases the proposal counter for a given channel by one.
///
/// # Panics
/// A panic occurrs if saving to the data store fails.
///
pub async fn bump_proposal_count(ctx: &Context, channel_id: ChannelId) -> u64 {
    let channel_id = *channel_id.as_u64();
    let client_data = ctx.data.read().await;

    let mut data = client_data
        .get::<DataStore>()
        .clone()
        .unwrap() // this should always be present
        .write()
        .await;

    let counter = data.store.channels.entry(channel_id).or_insert(0);
    *counter += 1;

    let counter = *counter;

    data.save().await.unwrap();
    counter
}

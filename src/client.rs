use std::sync::Arc;
use tokio::sync::RwLock;

use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::framework::StandardFramework;
use serenity::model::interactions::Interaction;
use serenity::{prelude::GatewayIntents, Client};

use crate::commands::GENERAL_GROUP;
use crate::data_store::DataStore;
use crate::interact::proposal;

struct Events;

#[async_trait]
impl EventHandler for Events {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::MessageComponent(mci) => match mci.data.custom_id.as_str() {
                proposal::INTERACT_ID_PROPOSAL_START => {
                    proposal::create_modal_response(&ctx, &mci).await.unwrap();
                }
                _ => (),
            },
            Interaction::ModalSubmit(mci) => match mci.data.custom_id.as_str() {
                proposal::INTERACT_ID_PROPOSAL_MODAL => {
                    proposal::handle_modal_submission(&ctx, &mci).await.unwrap();
                }
                _ => (),
            },
            _ => (),
        }
    }
}

pub async fn start(token: String) -> Result<(), Box<dyn std::error::Error>> {
    let intents = GatewayIntents::non_privileged()
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(token, intents)
        .event_handler(Events)
        .framework(framework)
        .await?;

    register_data_store(&client).await;
    register_shutdown(&client);

    client.start().await?;
    Ok(())
}

async fn register_data_store(client: &Client) {
    let mut data = client.data.write().await;

    data.insert::<DataStore>(Arc::new(RwLock::new(
        DataStore::new("store.json").await.unwrap(),
    )));
}

fn register_shutdown(client: &Client) {
    let shard_mgr = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        shard_mgr.lock().await.shutdown_all().await;
    });
}

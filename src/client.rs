use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::framework::StandardFramework;
use serenity::model::gateway::Ready;
use serenity::model::interactions::Interaction;
use serenity::{prelude::GatewayIntents, Client};
use tracing::info;

use crate::commands::GENERAL_GROUP;
use crate::interact;

struct Events;

#[async_trait]
impl EventHandler for Events {
    // called on init
    async fn ready(&self, _: Context, ready: Ready) {
        let uid = ready.user.id;
        info!("connected: {}", uid);

        info!(
            "https://discord.com/oauth2/authorize?client_id={}&permissions=0&scope=bot",
            uid
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::MessageComponent(mci) => match mci.data.custom_id.as_str() {
                interact::proposal::INTERACT_ID_PROPOSAL_START => {
                    interact::proposal::create_modal_response(&ctx, &mci)
                        .await
                        .unwrap()
                }
                _ => (),
            },
            _ => (),
        }
    }
}

fn register_shutdown(client: &Client) {
    let shard_mgr = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        shard_mgr.lock().await.shutdown_all().await;
    });
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

    register_shutdown(&client);

    client.start().await?;
    Ok(())
}

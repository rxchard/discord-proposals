use discord_proposals::client;
use tracing::error;

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").unwrap();
    tracing_subscriber::fmt::init();

    if let Err(err) = client::start(token).await {
        error!(err);
    }
}

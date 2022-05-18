use serenity::{
    client::Context,
    model::{
        channel::Message,
        id::ChannelId,
        interactions::{
            message_component::{
                ActionRowComponent, ButtonStyle, InputTextStyle, MessageComponentInteraction,
            },
            modal::ModalSubmitInteraction,
            InteractionResponseType,
        },
    },
    Result as SerenityResult,
};

use tracing::debug;

use super::util;

pub const INTERACT_ID_PROPOSAL_START: &str = "interact_proposal_start";
pub const INTERACT_ID_PROPOSAL_MODAL: &str = "interact_proposal_modal";

async fn create_proposal(
    ctx: &Context,
    text_input: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let channel_id = std::env::var("DISCORD_CHANNEL")?; // could add another data entry for environment variables
    let channel_id = ChannelId(channel_id.parse()?);

    let proposal_message = channel_id.say(ctx, text_input).await?;

    // attach a thread
    let thread_id = util::bump_proposal_count(ctx, channel_id).await;
    let thread_name = format!("[PalmP{}] Discussion", thread_id);

    channel_id
        .create_public_thread(ctx, proposal_message.id, |thr| {
            thr.name(thread_name).auto_archive_duration(10080) // longest duration possible, one week
        })
        .await?;

    Ok(())
}

pub async fn handle_modal_submission(
    ctx: &Context,
    mci: &ModalSubmitInteraction,
) -> SerenityResult<()> {
    let ar = mci.data.components.get(0).unwrap();
    let component = ar.components.get(0).unwrap();

    if let ActionRowComponent::InputText(text_input) = component {
        let text_input = text_input.value.clone();

        // reply with a loading message
        mci.create_interaction_response(ctx, |f| {
            f.kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(|d| d.ephemeral(true))
        })
        .await?;

        let message = match create_proposal(ctx, text_input).await {
            Ok(_) => "Created your proposal successfully.",
            Err(_) => "An error occurred.",
        };

        // update deferred message
        mci.edit_original_interaction_response(&ctx, |m| m.content(message))
            .await?;
    }

    Ok(())
}

pub async fn create_message_in_channel(
    ctx: &Context,
    channel_id: &ChannelId,
) -> SerenityResult<Message> {
    debug!("creating interaction message in {}", channel_id);

    channel_id
        .send_message(&ctx, |m| {
            m.content("Create a new proposal for PalmDAO")
                .components(|c| {
                    c.create_action_row(|ar| {
                        ar.create_button(|button| {
                            button
                                .style(ButtonStyle::Primary)
                                .label("Submit Proposal")
                                .custom_id(INTERACT_ID_PROPOSAL_START)
                        })
                    })
                })
        })
        .await
}

pub async fn create_modal_response(
    ctx: &Context,
    mci: &MessageComponentInteraction,
) -> SerenityResult<()> {
    debug!("creating modal in {}", mci.channel_id);

    mci.create_interaction_response(&ctx, |re| {
        re.kind(InteractionResponseType::Modal)
            .interaction_response_data(|data| {
                data.custom_id(INTERACT_ID_PROPOSAL_MODAL)
                    .title("Proposal")
                    .components(|c| {
                        c.create_action_row(|row| {
                            row.create_input_text(|text_input| {
                                text_input
                                    .label("Describe your idea...")
                                    .custom_id("proposal_input")
                                    .placeholder("Your proposal")
                                    .style(InputTextStyle::Paragraph)
                                    .max_length(1950)
                                    .required(true)
                            })
                        })
                    })
            })
    })
    .await
}

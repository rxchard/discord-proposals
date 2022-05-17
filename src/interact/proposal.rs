use serenity::{
    client::Context,
    model::{
        channel::Message,
        id::ChannelId,
        interactions::{
            message_component::{ButtonStyle, InputTextStyle, MessageComponentInteraction},
            InteractionResponseType,
        },
    },
    Result,
};
use tracing::debug;

pub const INTERACT_ID_PROPOSAL_START: &str = "interact_proposal_start";
pub const INTERACT_ID_PROPOSAL_MODAL: &str = "interact_proposal_modal";

pub async fn create_message_in_channel(ctx: &Context, channel_id: &ChannelId) -> Result<Message> {
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

pub async fn create_modal_response(ctx: &Context, mci: &MessageComponentInteraction) -> Result<()> {
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
                                    .required(true)
                            })
                        })
                    })
            })
    })
    .await
}

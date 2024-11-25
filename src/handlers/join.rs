use serenity::all::{
    ChannelId, Colour, Context, CreateAllowedMentions, CreateEmbed, CreateEmbedFooter,
    CreateMessage, Member, Mentionable,
};
use std::error::Error;
use tracing::{error, warn};

#[tracing::instrument(skip(ctx, new_member, data))]
pub async fn join_handler(
    ctx: Context,
    data: &crate::Data,
    new_member: Member,
) -> Result<(), Box<dyn Error>> {
    let welcome_msg = CreateEmbed::default()
        .title("Welcome to sillycord!")
        .description(format!(
            "Welcome! To keep sillycord a safe and fun place, we require all newly invited members to be vouched by a current member. Please wait for a member of the community to vouch for you. If you do not receive a vouch within 24 hours, you will be removed from the server. If you have any questions, feel free to ask a moderator or admin.",
        ))
        .color(Colour::PURPLE)
        .footer(CreateEmbedFooter::new(
            "I am a bot, and this action was performed automatically. If you have any questions or concerns, please contact a moderator or admin.",
        ));

    // Try to send the welcome message to the new member
    let dm_result = new_member
        .user
        .direct_message(&ctx, CreateMessage::default().embed(welcome_msg.clone()))
        .await;

    if let Err(why) = dm_result {
        warn!(
            "Failed to send welcome message to {}: {:?} - defaulting to public channel",
            new_member.user.tag(),
            why
        );

        let new_welcome = welcome_msg.clone().footer(CreateEmbedFooter::new(
            "We were unable to send you a direct message. We've posted the welcome message here instead - please make sure to read it!",
        ));

        let send_result = ctx
            .http
            .send_message(
                ChannelId::new(data.config.channels.welcome),
                vec![],
                &CreateMessage::default()
                    .embed(new_welcome)
                    .content(format!(
                        "{} Please make sure to read the welcome message below!",
                        new_member.mention()
                    ))
                    .allowed_mentions(CreateAllowedMentions::new().users(vec![new_member.user.id])),
            )
            .await;

        if let Err(why) = send_result {
            error!(
                "Failed to send welcome message to {} in public channel: {:?}",
                new_member.user.tag(),
                why
            );

            error!(
                "All options to send welcome message to {} failed - aborting",
                new_member.user.tag()
            );

            return Err(why.into());
        }
    }

    Ok(())
}

use crate::{handlers, utils::get_rustc_version, Data, Error};
use ::serenity::all::{
    ChannelId, Colour, CreateEmbed, CreateEmbedFooter, CreateMessage, Mentionable,
};
use poise::serenity_prelude::{self as serenity, ActivityData, OnlineStatus};
use tracing::{error, info};

// Create a span for every event
#[tracing::instrument(skip(ctx, event, _framework, data))]
pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { .. } => {
            info!("Bot is ready");
            ctx.set_presence(
                Some(ActivityData::watching("sillycord")),
                OnlineStatus::Online,
            );

            let embed = CreateEmbed::default()
                .title("Bot Ready!")
                .description(format!(
                    "Bot is ready! Running on rust version {}, sillycord-bot version {}",
                    get_rustc_version().await,
                    env!("CARGO_PKG_VERSION")
                ))
                .footer(CreateEmbedFooter::new(
                    "Bot created for sillycord. Made with ❤️ by sticks and others",
                ))
                .color(Colour::DARK_GREEN);

            let msg = CreateMessage::default().embed(embed);
            let channel_id = ChannelId::new(data.config.channels.logs_public);

            // We have to use http here to be future safe and not block the event loop
            ctx.http.send_message(channel_id, vec![], &msg).await?;
        }

        serenity::FullEvent::GuildMemberAddition { new_member, .. } => {
            // Is the new user in the main guild?
            if new_member.guild_id != data.config.main_guild_id {
                return Ok(());
            }

            info!("Handling new user {}", new_member.user.tag());

            ctx.http
                .send_message(
                    ChannelId::new(data.config.channels.logs_mod),
                    vec![],
                    &CreateMessage::default().content(format!(
                        "<:join:1310407968503894158> New user joined {} - created at: <t:{}:f>",
                        new_member.mention(),
                        new_member.user.created_at().timestamp_millis() / 1000
                    )),
                )
                .await?;

            let handler_result =
                handlers::join::join_handler(ctx.clone(), data, new_member.clone()).await;

            if let Err(e) = handler_result {
                error!(
                    "Error invoking join handler for new user {}: {:?}",
                    new_member.user.tag(),
                    e
                );
            } else {
                info!(
                    "Join handler completed successfully for new user {}",
                    new_member.user.tag()
                );
            }
        }

        serenity::FullEvent::GuildMemberRemoval { user, .. } => {
            // We can safely ignore the user's guild_id here, as they are leaving the guild -
            // because the bot is only in one guild, we know the user is leaving the main guild
            info!("Handling user leave {}", user.tag());

            data.database_controller
                .delete_user_by_discord_id(user.id.into())
                .await?;

            ctx.http
                .send_message(
                    ChannelId::new(data.config.channels.logs_mod),
                    vec![],
                    &CreateMessage::default().content(format!(
                        "<:leave:1310407968503894158> User left {}",
                        user.mention()
                    )),
                )
                .await?;
        }

        serenity::FullEvent::ShardsReady { total_shards, .. } => {
            info!("All shards emitted Ready, using {} shards", total_shards);
        }

        _ => {}
    }
    Ok(())
}

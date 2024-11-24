use crate::{Data, Error};
use poise::serenity_prelude::{self as serenity, ActivityData, Interaction, OnlineStatus};
use tracing::{info, warn};

// Create a span for every event
#[tracing::instrument(skip(ctx, event, framework, data))]
pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { .. } => {
            info!("Bot is ready");
            ctx.set_presence(
                Some(ActivityData::watching("sillycord")),
                OnlineStatus::Online,
            );
        }

        serenity::FullEvent::ShardsReady { total_shards, .. } => {
            info!("All shards emitted Ready, using {} shards", total_shards);
        }

        _ => {}
    }
    Ok(())
}

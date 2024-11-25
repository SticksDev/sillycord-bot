use poise::CreateReply;
use serenity::{all::CreateEmbed, model::colour};

use crate::{Context, Error};

/// Pong? Returns the ping of the bot and process uptime
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();

    // Create pining embed
    let mut embed = CreateEmbed::default()
        .title("Ping?")
        .description("Pinging...")
        .color(colour::Colour::PURPLE);

    let msg = ctx.send(CreateReply::default().embed(embed)).await?;

    let ping = start.elapsed().as_millis();
    embed = CreateEmbed::default()
        .title("Pong!")
        .description(format!(
            "Pong! Took {}ms!\nProcess uptime: {:?}",
            ping,
            // Round to 2 decimal places
            (ctx.data().uptime.elapsed().as_secs_f64() * 100.0).floor() / 100.0
        ))
        .color(colour::Colour::DARK_GREEN);

    msg.edit(ctx, CreateReply::default().embed(embed)).await?; // Edit the message with the ping
    Ok(())
}

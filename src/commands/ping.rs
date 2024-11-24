use crate::{Context, Error};

#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
    let start = std::time::Instant::now();
    let msg = ctx.say("Pong?").await?;

    let latency = start.elapsed();
    msg.edit(
        ctx,
        poise::CreateReply::default().content(format!("Pong! Latency: {:?}", latency)),
    )
    .await?;

    Ok(())
}

use crate::{Context, Error};
use poise::CreateReply;
use reqwest::Client;
use serde_json::Value;
use serenity::all::CreateAttachment;

/// Returns a random dog image.
#[poise::command(slash_command)]
pub async fn dog(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let dog_res = Client::new()
        .get("https://dog.ceo/api/breeds/image/random")
        .send()
        .await?;

    let dog: Value = serde_json::from_str(&dog_res.text().await?)?;
    let dog = dog["message"].as_str().unwrap();

    ctx.send(
        CreateReply::default()
            .content("Here's a random dog image:")
            .attachment(CreateAttachment::url(ctx.http(), dog).await?),
    )
    .await?;
    Ok(())
}

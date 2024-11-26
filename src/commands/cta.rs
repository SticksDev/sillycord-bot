use crate::{Context, Error};
use poise::CreateReply;
use reqwest::Client;
use serde_json::Value;
use serenity::all::CreateAttachment;

/// Returns a random cat image, along with a random cat fact.
#[poise::command(slash_command)]
pub async fn cta(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let cat_fact_res = Client::new()
        .get("https://catfact.ninja/fact")
        .send()
        .await?;

    let cat_fact: Value = serde_json::from_str(&cat_fact_res.text().await?)?;
    let cat_fact = cat_fact["fact"].as_str().unwrap();

    let cat_image_res = Client::new()
        .get("https://api.thecatapi.com/v1/images/search")
        .send()
        .await?;

    let cat_image: Value = serde_json::from_str(&cat_image_res.text().await?)?;
    let cat_image = cat_image[0]["url"].as_str().unwrap();

    ctx.send(
        CreateReply::default()
            .content(format!("*{}*", cat_fact))
            .attachment(CreateAttachment::url(ctx.http(), cat_image).await?),
    )
    .await?;
    Ok(())
}

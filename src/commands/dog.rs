use crate::{Context, Error};
use poise::CreateReply;
use reqwest::Client;
use serde_json::Value;
use serenity::{all::CreateEmbed, model::colour};

/// Returns a random dog image.
#[poise::command(slash_command)]
pub async fn dog(ctx: Context<'_>) -> Result<(), Error> {
    let dog_res = Client::new()
        .get("https://dog.ceo/api/breeds/image/random")
        .send()
        .await?;

    let dog: Value = serde_json::from_str(&dog_res.text().await?)?;
    let dog = dog["message"].as_str().unwrap();

    let embed = CreateEmbed::default()
        .title("Random Dog")
        .image(dog)
        .color(colour::Colour::from_rgb(0, 255, 255));

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}

use crate::{utils, Context, Error};
use poise::CreateReply;
use serenity::all::{CreateAllowedMentions, CreateAttachment, Message};
use serenity::model::prelude::UserId;
use serenity::prelude::Mentionable;

async fn check_if_allowed(ctx: Context<'_>, message: Message) -> Result<bool, Error> {
    let user = ctx
        .data()
        .database_controller
        .get_user_by_discord_id(message.author.id.into())
        .await?;

    if user.is_none() {
        ctx.data()
            .database_controller
            .create_user(message.author.id.into())
            .await?;
    }

    let user = ctx
        .data()
        .database_controller
        .get_user_by_discord_id(message.author.id.into())
        .await?;

    Ok(user.unwrap().actions_allowed)
}

#[poise::command(context_menu_command = "Hug User")]
pub async fn use_action_hug(
    ctx: Context<'_>,
    #[description = "The target message to use the action with"] message: Message,
) -> Result<(), Error> {
    ctx.defer().await?;

    let allowed = check_if_allowed(ctx, message.clone()).await?;

    if !allowed {
        ctx.send(CreateReply::default().content(
            ":x: Sorry, either that user has disabled actions or has no profile to check against",
        ).ephemeral(true))
        .await?;
        return Ok(());
    }

    let action_img = utils::get_random_action_image("hug".to_string()).await;

    // Match the result of the action image
    match action_img {
        Ok(img) => {
            let user = UserId::new(ctx.author().id.into());
            let target = UserId::new(message.clone().author.id.into());
            let builder = CreateReply::default()
                .content(format!("{} hugs {}", user.mention(), target.mention()))
                .attachment(CreateAttachment::url(ctx.http(), &img).await?)
                .allowed_mentions(CreateAllowedMentions::default().users(vec![user, target]));

            ctx.send(builder).await?;
        }
        Err(_) => {
            ctx.send(
                CreateReply::default()
                    .content(":x: Something went wrong while fetching the action image")
                    .ephemeral(true),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(context_menu_command = "Kiss User")]
pub async fn use_action_kiss(
    ctx: Context<'_>,
    #[description = "The target message to use the action with"] message: Message,
) -> Result<(), Error> {
    ctx.defer().await?;
    let allowed = check_if_allowed(ctx, message.clone()).await?;

    if !allowed {
        ctx.send(CreateReply::default().content(
            ":x: Sorry, either that user has disabled actions or has no profile to check against",
        ).ephemeral(true))
        .await?;
        return Ok(());
    }

    let action_img = utils::get_random_action_image("kiss".to_string()).await;

    // Match the result of the action image
    match action_img {
        Ok(img) => {
            let user = UserId::new(ctx.author().id.into());
            let target = UserId::new(message.author.id.into());
            let builder = CreateReply::default()
                .content(format!("{} kisses {}", user.mention(), target.mention()))
                .attachment(CreateAttachment::url(ctx.http(), &img).await?)
                .allowed_mentions(CreateAllowedMentions::default().users(vec![user, target]));

            ctx.send(builder).await?;
        }
        Err(_) => {
            ctx.send(
                CreateReply::default()
                    .content(":x: Something went wrong while fetching the action image")
                    .ephemeral(true),
            )
            .await?;
        }
    }

    Ok(())
}

#[poise::command(context_menu_command = "Pat User")]
pub async fn use_action_pat(
    ctx: Context<'_>,
    #[description = "The target message to use the action with"] message: Message,
) -> Result<(), Error> {
    ctx.defer().await?;
    let allowed = check_if_allowed(ctx, message.clone()).await?;

    if !allowed {
        ctx.send(CreateReply::default().content(
            ":x: Sorry, either that user has disabled actions or has no profile to check against",
        ).ephemeral(true))
        .await?;
        return Ok(());
    }

    let action_img = utils::get_random_action_image("pat".to_string()).await;

    // Match the result of the action image
    match action_img {
        Ok(img) => {
            let user = UserId::new(ctx.author().id.into());
            let target = UserId::new(message.author.id.into());
            let builder = CreateReply::default()
                .content(format!("{} pats {}", user.mention(), target.mention()))
                .attachment(CreateAttachment::url(ctx.http(), &img).await?)
                .allowed_mentions(CreateAllowedMentions::default().users(vec![user, target]));

            ctx.send(builder).await?;
        }
        Err(_) => {
            ctx.send(
                CreateReply::default()
                    .content(":x: Something went wrong while fetching the action image")
                    .ephemeral(true),
            )
            .await?;
        }
    }

    Ok(())
}

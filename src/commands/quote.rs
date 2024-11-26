use crate::{Context, Error};
use poise::CreateReply;
use serenity::all::{CreateAllowedMentions, Message, User};
use sqlx::types::time::OffsetDateTime;

#[poise::command(context_menu_command = "Quote User")]
pub async fn quote_action(
    ctx: Context<'_>,
    #[description = "The target message to quote"] message: Message,
) -> Result<(), Error> {
    let quote = crate::structs::quote::Quote {
        quote_id: 0,
        user_id: message.author.id.into(),
        username: message.author.name.clone(),
        quote: message.content.clone(),
        added_by: ctx.author().id.into(),
        added_at: Some(OffsetDateTime::now_utc()),
    };

    if quote.user_id == quote.added_by {
        ctx.say(":x: You can't quote yourself").await?;
        return Ok(());
    }

    ctx.data().database_controller.quote_create(quote).await?;
    ctx.say(":white_check_mark: Okay, I've immortalized that message for you :)")
        .await?;
    Ok(())
}

/// Get a random quote
#[poise::command(slash_command)]
pub async fn random_quote(ctx: Context<'_>) -> Result<(), Error> {
    let quote = ctx.data().database_controller.quote_get_random().await?;

    if let Some(quote) = quote {
        ctx.send(
            CreateReply::default()
                .content(format!(
                    "{}: {}\nQuoted at: <t:{}:f> by <@{}>",
                    quote.username,
                    quote.quote,
                    if let Some(added_at) = quote.added_at {
                        added_at.unix_timestamp()
                    } else {
                        // Use now if added_at is None
                        OffsetDateTime::now_utc().unix_timestamp()
                    },
                    quote.added_by
                ))
                .allowed_mentions(CreateAllowedMentions::new().empty_users()),
        )
        .await?;
    } else {
        ctx.say("No quotes found").await?;
    }

    Ok(())
}

/// Get quotes for a user, showing latest 10
#[poise::command(slash_command)]
pub async fn user_quotes(
    ctx: Context<'_>,
    #[description = "The user to get quotes for"] user: User,
) -> Result<(), Error> {
    let quotes = ctx
        .data()
        .database_controller
        .quote_get_by_user_id(user.id.into())
        .await?;

    if quotes.is_empty() {
        ctx.say("No quotes found").await?;
        return Ok(());
    }

    let mut response = String::new();
    for quote in quotes {
        response.push_str(&format!(
            "{}: {}\nQuoted at: <t:{}:f> by <@{}>\n",
            quote.username,
            quote.quote,
            if let Some(added_at) = quote.added_at {
                added_at.unix_timestamp()
            } else {
                // Use now if added_at is None
                OffsetDateTime::now_utc().unix_timestamp()
            },
            quote.added_by
        ));
    }

    ctx.send(
        CreateReply::default()
            .content(response)
            .allowed_mentions(CreateAllowedMentions::new().empty_users()),
    )
    .await?;
    Ok(())
}

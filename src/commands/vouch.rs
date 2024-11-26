use crate::{structs::vouch::Vouch, Context, Error};
use serenity::all::{
    Colour, CreateAllowedMentions, CreateEmbed, CreateEmbedFooter, CreateMessage, Mentionable, User,
};

/// Commands related to vouching for new users
#[poise::command(slash_command, subcommands("submit", "approve", "deny"))]
pub async fn vouch(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Vouch for a new user
#[poise::command(slash_command, guild_only)]
pub async fn submit(
    ctx: Context<'_>,
    #[description = "The user to vouch for"] user: User,
) -> Result<(), Error> {
    // Defer emphemeral response
    ctx.defer_ephemeral().await?;

    // Do we have a vouch for this user already?
    if ctx
        .data()
        .vouch_store
        .lock()
        .await
        .iter()
        .any(|vouch| vouch.user.id == user.id)
    {
        ctx.say(":x: This user already has a vouch pending!")
            .await?;
        return Ok(());
    }

    let guild_id = ctx.guild_id().ok_or("Must be run in a guild")?;
    let member = guild_id.member(ctx.serenity_context(), user.id).await?;

    // Does the user have a silly role?
    if member
        .roles
        .iter()
        .any(|role_id| *role_id == ctx.data().config.roles.silly_role)
    {
        ctx.say(":x: This user already is a vouched member!")
            .await?;
        return Ok(());
    }

    // Create a new vouch
    let vouch = Vouch::new(user.clone(), ctx.author().clone());

    // Add the vouch to the store, we need to create a mutable reference to .data().vouch_store,
    // because we want to modify the store - and poise context is immutable by default
    ctx.data().vouch_store.lock().await.push(vouch);

    // Send a messasge to the mod-logs channel with a ping that a new vouch has been submitted
    let log_msg = format!(
        "{}\n:notepad_spiral: A new vouch has been submitted for {} by {}, please either approve or deny this vouch.",
        serenity::model::id::RoleId::new(ctx.data().config.roles.admin).mention(),
        user.clone().mention(),
        ctx.author().mention()
    );

    let log_channel_id = ctx.data().config.channels.logs_mod;
    let channel_hash = guild_id.channels(ctx.serenity_context()).await?;
    let channel = channel_hash.get(&log_channel_id.into()).unwrap();

    channel
        .send_message(
            ctx.serenity_context(),
            CreateMessage::new().content(log_msg).allowed_mentions(
                CreateAllowedMentions::new().roles(vec![ctx.data().config.roles.admin]),
            ),
        )
        .await?;

    ctx.say(":white_check_mark: Vouch submitted! An admin will review the vouch when able.")
        .await?;

    Ok(())
}

/// Approve a vouch for a user
#[poise::command(slash_command, guild_only)]
pub async fn approve(
    ctx: Context<'_>,
    #[description = "The user to approve the vouch for"] user: User,
) -> Result<(), Error> {
    // Defer emphemeral response
    ctx.defer_ephemeral().await?;

    // Grab user
    let author_user = ctx.author_member().await.clone();
    match author_user {
        Some(author_user) => {
            // Check if the author is an admin
            if !author_user
                .roles
                .iter()
                .any(|role_id| *role_id == ctx.data().config.roles.admin)
            {
                ctx.say(":x: You must be an admin to approve vouches!")
                    .await?;
                return Ok(());
            }
        }
        None => {
            ctx.say(":x: You must be an admin to approve vouches!")
                .await?;
            return Ok(());
        }
    }

    // Do we have a vouch for this user?
    let mut vouch_store = ctx.data().vouch_store.lock().await;

    // Find the vouch for the user and its index
    if let Some((vouch_index, vouch)) = vouch_store
        .iter()
        .enumerate()
        .find(|(_, v)| v.user.id == user.id)
        .map(|(index, vouch)| (index, vouch.clone()))
    {
        // Add the silly role to the user
        let guild_id = ctx.guild_id().ok_or("Must be run in a guild")?;
        let member = guild_id.member(ctx.serenity_context(), user.id).await?;

        member
            .add_role(ctx.serenity_context(), ctx.data().config.roles.silly_role)
            .await?;

        // Remove the vouch from the store
        vouch_store.remove(vouch_index);

        // Send a message to the logs channel (mod)
        let log_msg = format!(
            ":white_check_mark: Vouch approved for {} by {} at {} CDT, vouched by {}",
            user.mention(),
            ctx.author().mention(),
            vouch.get_vouch_time(),
            vouch.vouched_by.mention()
        );

        let public_msg = CreateEmbed::default()
            .title(format!("Welcome to sillycord, {}!", user.tag()))
            .description("Enjoy your stay in our silly little community!")
            .footer(CreateEmbedFooter::new(format!(
                "Vouched by {} - Approved by {}",
                vouch.vouched_by.tag(),
                ctx.author().tag()
            )))
            .color(Colour::DARK_PURPLE);

        let log_channel_id = ctx.data().config.channels.logs_mod;
        let public_channel_id = ctx.data().config.channels.main;

        let channels = guild_id.channels(ctx.serenity_context()).await?;
        let channel = channels.get(&log_channel_id.into()).unwrap();
        let public_channel = channels.get(&public_channel_id.into()).unwrap();

        channel
            .send_message(
                ctx.serenity_context(),
                CreateMessage::new().content(log_msg),
            )
            .await?;

        public_channel
            .send_message(
                ctx.serenity_context(),
                CreateMessage::new()
                    .embed(public_msg)
                    .content(user.mention().to_string())
                    .allowed_mentions(CreateAllowedMentions::new().users(vec![user.id])),
            )
            .await?;

        ctx.data()
            .database_controller
            .create_user(user.id.into())
            .await?;
        ctx.say(":white_check_mark: Vouch approved!").await?;
    } else {
        ctx.say(":x: No vouch found for this user!").await?;
    }

    drop(vouch_store);
    Ok(())
}

/// Deny a vouch for a user
#[poise::command(slash_command, guild_only)]
pub async fn deny(
    ctx: Context<'_>,
    #[description = "The user to deny the vouch for"] user: User,
    #[flag] kick: bool,
    #[description = "The reason for denying the vouch"] _reason: Option<String>,
) -> Result<(), Error> {
    // Defer emphemeral response
    ctx.defer_ephemeral().await?;

    // Grab user
    let author_user = ctx.author_member().await.clone();
    match author_user {
        Some(author_user) => {
            // Check if the author is an admin
            if !author_user
                .roles
                .iter()
                .any(|role_id| *role_id == ctx.data().config.roles.admin)
            {
                ctx.say(":x: You must be an admin to deny vouches!").await?;
                return Ok(());
            }
        }
        None => {
            ctx.say(":x: You must be an admin to deny vouches!").await?;
            return Ok(());
        }
    }

    // Do we have a vouch for this user?
    // Do we have a vouch for this user?
    let mut vouch_store = ctx.data().vouch_store.lock().await;

    // Find the vouch for the user and its index
    if let Some((vouch_index, vouch)) = vouch_store
        .iter()
        .enumerate()
        .find(|(_, v)| v.user.id == user.id)
        .map(|(index, vouch)| (index, vouch.clone()))
    {
        // Remove the vouch from the store
        vouch_store.remove(vouch_index);

        // Send a message to the logs channel (mod)
        let log_msg = format!(
            ":x: Vouch denied for {} by {} at {} CDT, vouched by {} with a reason of '{}'",
            user.mention(),
            ctx.author().mention(),
            vouch.get_vouch_time(),
            vouch.vouched_by.mention(),
            _reason.clone().unwrap_or("No reason provided".to_string())
        );

        let log_channel_id = ctx.data().config.channels.logs_mod;
        let guild = ctx.guild_id().unwrap().clone();
        let channels = guild.channels(ctx.serenity_context()).await?;
        let channel = channels.get(&log_channel_id.into()).unwrap();

        channel
            .send_message(
                ctx.serenity_context(),
                CreateMessage::new().content(log_msg),
            )
            .await?;

        // Then kick the user
        let member = guild.member(ctx.serenity_context(), user.id).await?;

        // Attempt to notify then kick the user
        let dm_msg = format!(
            ":warning: Your vouch has been denied by {} with a reason of '{}'. If you have any questions, feel free to ask a moderator or admin.",
            ctx.author().mention(),
            _reason.unwrap_or("No reason provided".to_string())
        );

        let notify_result = vouch
            .user
            .direct_message(ctx.serenity_context(), CreateMessage::new().content(dm_msg))
            .await;

        match notify_result {
            Ok(_) => {
                if kick {
                    member.kick(ctx.serenity_context()).await?;
                    ctx.say(":white_check_mark: Vouch denied and user kicked! User notified.")
                        .await?;
                } else {
                    ctx.say(":white_check_mark: Vouch denied! User notified.")
                        .await?;
                }
            }
            Err(_) => {
                if kick {
                    member.kick(ctx.serenity_context()).await?;
                    ctx.say(":white_check_mark: Vouch denied and user kicked! User not notified.")
                        .await?;
                } else {
                    ctx.say(":white_check_mark: Vouch denied! User not notified.")
                        .await?;
                }
            }
        }
    } else {
        ctx.say(":x: No vouch found for this user!").await?;
    }

    drop(vouch_store);
    Ok(())
}

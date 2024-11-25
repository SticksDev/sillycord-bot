use crate::structs::user::User as UserStruct;
use crate::{Context, Error};
use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed, User};

/// Commands related to profiles in the bot
#[poise::command(
    slash_command,
    subcommands("view", "edit"),
    check = "ensure_profile_is_setup"
)]
pub async fn profiles(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Ensure the user has a profile setup
/// If the user doesn't have a profile, create one
async fn ensure_profile_is_setup(ctx: Context<'_>) -> Result<bool, Error> {
    let profile = ctx
        .data()
        .database_controller
        .get_user_by_discord_id(ctx.author().id.into())
        .await?;

    if profile.is_none() {
        ctx.data()
            .database_controller
            .create_user(ctx.author().id.into())
            .await?;
    }

    Ok(true)
}

/// View a user's profile
#[poise::command(slash_command)]
pub async fn view(
    _ctx: Context<'_>,
    #[description = "The user to view the profile of, defaults to yourself"] user: Option<User>,
) -> Result<(), Error> {
    // Get the profile of the provided user, or the user who ran the command
    let target_user = user.unwrap_or_else(|| _ctx.author().clone());

    // Get the profile of the target user
    let profile = _ctx
        .data()
        .database_controller
        .get_user_by_discord_id(target_user.id.into())
        .await?;

    match profile {
        Some(profile) => {
            let profile_embed = CreateEmbed::default()
                .title(format!("Profile of {}", target_user.tag()))
                .description(format!(
                    "About: {}\nPronouns: {}\nActions Allowed: {}",
                    profile.about.unwrap_or("No about section".to_string()),
                    profile.pronouns.unwrap_or("No pronouns set".to_string()),
                    if profile.actions_allowed { "Yes" } else { "No" }
                ))
                .color(Colour::FABLED_PINK);

            _ctx.send(CreateReply::default().embed(profile_embed))
                .await?;
        }
        None => {
            // Send a emphul messagrespond(formate if the user doesn't have a profile
            _ctx.send(
                CreateReply::default()
                    .content(":x: User has no profile")
                    .ephemeral(true),
            )
            .await?;
        }
    }

    Ok(())
}

/// Edit your profile
#[poise::command(slash_command)]
pub async fn edit(
    _ctx: Context<'_>,
    #[description = "Your about section"] about: Option<String>,
    #[description = "Your pronouns"] pronouns: Option<String>,
    #[description = "Whether you want to allow actions to be performed on you"]
    actions_allowed: Option<bool>,
) -> Result<(), Error> {
    // If no options are provided, send a help message
    if about.is_none() && pronouns.is_none() && actions_allowed.is_none() {
        _ctx.send(
            CreateReply::default()
                .content("Please provide at least one option to edit")
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    // Get the profile of the user who ran the command
    let profile = _ctx
        .data()
        .database_controller
        .get_user_by_discord_id(_ctx.author().id.into())
        .await?;

    // If the user doesn't have a profile, create one
    let profile = match profile {
        Some(profile) => profile,
        None => {
            _ctx.data()
                .database_controller
                .create_user(_ctx.author().id.into())
                .await?
        }
    };

    // Generate a user struct with the updated values
    let updated_profile = UserStruct {
        id: profile.id,
        discord_id: profile.discord_id,
        about,
        pronouns,
        actions_allowed: actions_allowed.unwrap_or(profile.actions_allowed),
    };

    // Update the user's profile
    _ctx.data()
        .database_controller
        .update_user(updated_profile)
        .await?;

    // Send a success message
    _ctx.send(
        CreateReply::default()
            .content(":white_check_mark: Profile updated successfully!")
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

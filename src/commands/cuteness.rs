use poise::CreateReply;
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg32;
use serenity::all::{CreateAllowedMentions, User};

use crate::{Context, Error};

/// Determine a user's cuteness
#[poise::command(slash_command)]
pub async fn cutenesss(
    ctx: Context<'_>,
    #[description = "The user to check"] user: User,
) -> Result<(), Error> {
    let thinking_messages = [
        ":thinking: Thinking super hard...",
        ":gear: Calculating...",
        ":bar_chart: Crunching the numbers...",
        ":bulb: Looking at the data...",
        ":mag: Investigating...",
        ":clipboard: Finalizing the report...",
    ];

    let thinking_message = ctx.say(thinking_messages[0]).await?;

    // Iterate through thinking messages with delays
    for i in 1..thinking_messages.len() {
        let wait_time = rand::thread_rng().gen_range(500..1501);

        // Wait time logic for last message
        if i == thinking_messages.len() - 1 {
            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
        } else {
            tokio::time::sleep(std::time::Duration::from_millis(wait_time)).await;
        }

        thinking_message
            .edit(ctx, CreateReply::default().content(thinking_messages[i]))
            .await?;
    }

    // Create a seeded RNG using the user's ID
    let mut rng = Pcg32::seed_from_u64(user.id.into());

    // Generate random factors
    let charm_factor = rng.gen_range(0.2..0.5);
    let randomness_factor = rng.gen_range(0.1..0.3);
    let humor_factor = rng.gen_range(0.05..0.2);

    // Calculate the cuteness score
    let mut cuteness_score: f64 = charm_factor + randomness_factor + humor_factor;

    // Normalize to 0.0 - 1.0 range (clamp if it exceeds 1.0 due to summing)
    cuteness_score = cuteness_score.powf(0.5); // Adding non-linearity
    cuteness_score = cuteness_score.min(1.0); // Ensure it's within range 0.0 to 1.0

    // Format as a percentage
    cuteness_score *= 100.0;

    // Final message
    thinking_message
        .edit(
            ctx,
            CreateReply::default()
                .content(format!(
                    "I have determined that {} is {:.2}% cute!",
                    user.name, cuteness_score
                ))
                .allowed_mentions(CreateAllowedMentions::default().users(vec![user.id])),
        )
        .await?;

    Ok(())
}

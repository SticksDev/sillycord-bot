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

    let current_thinking_index = 0;
    let thinking_message = ctx.say(thinking_messages[current_thinking_index]).await?;

    // Wait between .5 and 1.5 seconds for each message, then edit the message with the next thinking message
    // On the last message, edit the message with the final message and wait 1.5 second
    for i in 1..thinking_messages.len() {
        let wait_time = rand::thread_rng().gen_range(500..1501);

        // Are we on the last message?
        if i == thinking_messages.len() - 1 {
            tokio::time::sleep(std::time::Duration::from_millis(1500)).await;
        } else {
            tokio::time::sleep(std::time::Duration::from_millis(wait_time)).await;
        }

        thinking_message
            .edit(ctx, CreateReply::default().content(thinking_messages[i]))
            .await?;
    }

    // Convert the users id to a byte array;
    let mut rng = Pcg32::seed_from_u64(user.id.into());
    let charm_factor = rng.gen_range(20..50);
    let randomness_factor = rng.gen_range(10..30);
    let humor_factor = rng.gen_range(5..20);

    // Convert the users id to a i64
    let user_id: i64 = user.id.into();
    let id_factor = user_id % 100;
    let cuteness_score = (charm_factor + randomness_factor + humor_factor + id_factor) % 101;

    thinking_message
        .edit(
            ctx,
            CreateReply::default()
                .content(format!(
                    "I have determined that {} is {}% cute!",
                    user.name, cuteness_score
                ))
                .allowed_mentions(CreateAllowedMentions::default().users(vec![user.id])),
        )
        .await?;

    Ok(())
}

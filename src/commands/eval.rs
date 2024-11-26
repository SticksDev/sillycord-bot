use std::{
    env, fs,
    process::{Command, Output, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

use poise::CreateReply;
use serenity::all::GetMessages;
use tracing::{info, warn};

use crate::{Context, Error};

/// Evaluate rust code
#[poise::command(prefix_command)]
pub async fn eval(
    ctx: Context<'_>,
    #[description = "The code to run"] code: poise::CodeBlock,
) -> Result<(), Error> {
    let authed_users = ctx.data().owners.clone();

    // Check if the user is an owner
    if !authed_users.contains(&ctx.author().id.into()) {
        let channel_id = ctx.channel_id();

        // Get messages in the channel (up to 100)
        let messages = channel_id
            .messages(&ctx, GetMessages::new().limit(100))
            .await?;

        // Find the most recent message that is not a bot message, and starts with ~eval
        let last_eval = messages.iter().find(|msg| {
            !msg.author.bot && msg.content.starts_with("~eval") && msg.author.id == ctx.author().id
        });

        // If the user has already run an eval command, delete the message
        if last_eval.is_some() {
            let _ = last_eval.unwrap().delete(&ctx).await;
            info!(
                "User {} tried to run eval without permission, and I deleted the message",
                ctx.author().id,
            );
        } else {
            warn!(
                "User {} tried to run eval without permission, but I couldn't delete the message",
                ctx.author().id
            );
        }

        return Ok(());
    }

    let org_msg = ctx.say(":gear: Processing...").await?;

    // Create a temporary directory for the file
    let temp_dir = env::temp_dir();
    let unique_id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let file_path = temp_dir.join(format!("eval_{}.rs", unique_id));
    let executable_path = temp_dir.join(format!("eval_{}.out", unique_id));

    // Write the code to a temporary file
    if let Err(err) = fs::write(&file_path, code.code) {
        org_msg
            .edit(
                ctx,
                CreateReply::default().content(format!(
                    "<:error:1310650177056538655> Error writing to file: {}",
                    err
                )),
            )
            .await?;
        return Ok(());
    }

    // Compile the Rust file and capture stderr
    let compile_output: Result<Output, _> = Command::new("rustc")
        .arg(&file_path)
        .arg("-o")
        .arg(&executable_path)
        .stderr(Stdio::piped())
        .output();

    match compile_output {
        Ok(output) if output.status.success() => {
            org_msg
                .edit(
                    ctx,
                    CreateReply::default().content("<:success:1310650176037453834> Compilation successful. Running the executable..."),
                )
                .await?;

            // Run the compiled executable
            let output = Command::new(&executable_path).output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    // Only show stdout/stderr if they are not empty, edit original message
                    let msg = "<:success:1310650176037453834> Exection successful";
                    let msg = if !stdout.is_empty() {
                        format!("{}\n\n**stdout**:\n```\n{}\n```", msg, stdout)
                    } else {
                        msg.to_string()
                    };

                    let msg = if !stderr.is_empty() {
                        format!("{}\n\n**stderr**:\n```\n{}\n```", msg, stderr)
                    } else {
                        msg.to_string()
                    };

                    org_msg
                        .edit(ctx, CreateReply::default().content(msg))
                        .await?;
                }
                Err(err) => {
                    org_msg
                        .edit(
                            ctx,
                            CreateReply::default().content(format!(
                                "<:error:1310650177056538655> Error running executable: {}",
                                err
                            )),
                        )
                        .await?;
                }
            }
        }
        Ok(output) => {
            // If compilation failed, display the compiler error output
            let stderr = String::from_utf8_lossy(&output.stderr);
            org_msg
                .edit(
                    ctx,
                    CreateReply::default().content(format!(
                        "<:error:1310650177056538655> Compilation failed:\n```\n{}\n```",
                        stderr
                    )),
                )
                .await?;
        }
        Err(err) => {
            org_msg
                .edit(
                    ctx,
                    CreateReply::default().content(format!(
                        "<:error:1310650177056538655> Error invoking rustc: {}",
                        err
                    )),
                )
                .await?;
        }
    }

    // Clean up
    let _ = fs::remove_file(&file_path);
    let _ = fs::remove_file(&executable_path);

    Ok(())
}

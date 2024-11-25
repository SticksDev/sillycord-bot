use std::{
    env, fs,
    process::{Command, Output, Stdio},
    time::{SystemTime, UNIX_EPOCH},
};

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
        ctx.say(":x: You are not authorized to run this command")
            .await?;

        return Ok(());
    }

    ctx.say(":gear: Processing...").await?;

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
        ctx.say(format!("Error writing to temporary file: {}", err))
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
            ctx.say(
                "<:success:1310650176037453834> Compilation successful. Running the executable...",
            )
            .await?;

            // Run the compiled executable
            let output = Command::new(&executable_path).output();

            match output {
                Ok(output) => {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    ctx.say(format!(
                        "**Output:**\n```\n{}\n```\n**Errors:**\n```\n{}\n```",
                        if stdout.trim().is_empty() {
                            "<no output>"
                        } else {
                            stdout.trim()
                        },
                        if stderr.trim().is_empty() {
                            "<no errors>"
                        } else {
                            stderr.trim()
                        }
                    ))
                    .await?;
                }
                Err(err) => {
                    ctx.say(format!(
                        "<:error:1310650177056538655> Error running the executable: {}",
                        err
                    ))
                    .await?;
                }
            }
        }
        Ok(output) => {
            // If compilation failed, display the compiler error output
            let stderr = String::from_utf8_lossy(&output.stderr);
            ctx.say(format!(
                "<:error:1310650177056538655> Compilation failed:\n```\n{}\n```",
                stderr
            ))
            .await?;
        }
        Err(err) => {
            ctx.say(format!(
                "<:error:1310650177056538655> Error invoking rustc: {}",
                err
            ))
            .await?;
        }
    }

    // Clean up
    let _ = fs::remove_file(&file_path);
    let _ = fs::remove_file(&executable_path);

    Ok(())
}

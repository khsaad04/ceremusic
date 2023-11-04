use crate::{Context, Error};

/// Play music
#[poise::command(prefix_command, slash_command)]
pub async fn play(ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

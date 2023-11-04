use crate::{Context, Error};

/// Leave vc
#[poise::command(prefix_command, slash_command, aliases("disconnect"))]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();
    let has_handler = manager.get(ctx.guild_id().unwrap()).is_some();

    if has_handler {
        if let Err(e) = manager.remove(ctx.guild_id().unwrap()).await {
            ctx.say(format!("Failed: {:?}", e)).await?;
        }

        ctx.say("Left voice channel").await?;
    } else {
        ctx.say("Not in a voice channel").await?;
    }
    Ok(())
}

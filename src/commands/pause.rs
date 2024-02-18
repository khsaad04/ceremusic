use crate::{Context, Error};

/// Pause music
#[poise::command(prefix_command, slash_command)]
pub async fn pause(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let handler_lock = manager.get(guild_id).unwrap();
    let handler = handler_lock.lock().await;

    let _ = match handler.queue().current() {
        Some(track) => track.pause(),
        None => {
            ctx.say("Nothing is being played right now").await?;
            return Ok(());
        }
    };

    ctx.say("Paused the current track").await?;
    Ok(())
}

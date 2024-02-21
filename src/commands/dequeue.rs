use crate::commands::play::TrackMetaKey;
use crate::{Context, Error};

/// Remove a track from queue
#[poise::command(prefix_command, slash_command, aliases("rm", "dq"))]
pub async fn dequeue(ctx: Context<'_>, index: usize) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap();
    let call = manager.get(guild_id).unwrap();

    let handler = call.lock().await;

    let _ = handler.queue().dequeue(index).unwrap();

    ctx.say(format!("Removed the track on index `{index}`",))
        .await?;

    Ok(())
}

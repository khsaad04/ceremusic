use crate::commands::play::TrackMetaKey;
use crate::{Context, Error};

/// Get current queue
#[poise::command(prefix_command, slash_command)]
pub async fn dequeue(ctx: Context<'_>, index: u32) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap();
    let call = manager.get(guild_id).unwrap();

    let handler = call.lock().await;

    let _ = handler.queue().dequeue(index.try_into().unwrap()).unwrap();

    ctx.say(format!("Removed song from index `{index}`",))
        .await?;

    Ok(())
}

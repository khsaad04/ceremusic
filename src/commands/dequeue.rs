use crate::{Context, Error};

/// Remove a track from queue
#[poise::command(prefix_command, slash_command, aliases("rm", "dq"))]
pub async fn dequeue(ctx: Context<'_>, index: usize) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Could not retrieve guild_id");
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Failed to get serenity_context");
    let call = manager
        .get(guild_id)
        .expect("Failed to get songbird call instance");

    let handler = call.lock().await;

    let _ = handler.queue().dequeue(index).expect("Failed to dequeue");

    ctx.say(format!("Removed the track on index `{index}`",))
        .await?;

    Ok(())
}

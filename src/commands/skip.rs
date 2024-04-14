use crate::commands::play::TrackMetaKey;
use crate::{Context, Error};

/// Get current queue
#[poise::command(prefix_command, slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().expect("Could not retrieve guild_id");
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Failed to get serenity_context");
    let call = manager
        .get(guild_id)
        .expect("Failed to get songbird call instance");

    let handler = call.lock().await;

    let track = handler
        .queue()
        .current()
        .expect("Failed to get current track from queue");
    let track_map = track.typemap().read().await;
    let metadata = track_map
        .get::<TrackMetaKey>()
        .expect("Failed to get track metadata");

    handler
        .queue()
        .skip()
        .expect("Failed to skip track in queue");

    ctx.say(format!(
        "Skipped `{}`",
        metadata.title.as_ref().expect("Invalid reference")
    ))
    .await?;

    Ok(())
}

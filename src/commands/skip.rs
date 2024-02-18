use crate::commands::play::TrackMetaKey;
use crate::{Context, Error};

/// Get current queue
#[poise::command(prefix_command, slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap();
    let call = manager.get(guild_id).unwrap();

    let handler = call.lock().await;

    let track = handler.queue().current().unwrap();
    let track_map = track.typemap().read().await;
    let metadata = track_map.get::<TrackMetaKey>().unwrap();

    let queue = handler.queue().skip().unwrap();

    ctx.say(format!("Skipped `{}`", metadata.title.as_ref().unwrap()))
        .await?;

    Ok(())
}

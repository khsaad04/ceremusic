use crate::commands::play::TrackMetaKey;
use crate::{Context, Error};

/// Get current queue
#[poise::command(prefix_command, slash_command, aliases("np"))]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let manager = songbird::get(ctx.serenity_context()).await.unwrap();
    let call = manager.get(guild_id).unwrap();

    let mut message_string = String::new();
    let handler = call.lock().await;

    let queue = handler.queue().current_queue();
    for (idx, track) in queue.iter().enumerate() {
        let track_map = track.typemap().read().await;
        let metadata = track_map.get::<TrackMetaKey>().unwrap();
        message_string
            .push_str(format!("{}: {}\n", idx + 1, metadata.title.as_ref().unwrap()).as_ref())
    }

    ctx.say(message_string).await?;

    Ok(())
}

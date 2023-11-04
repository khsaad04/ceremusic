use crate::{Context, Error};
use songbird::{input::ytdl_search, ytdl};

/// Play music
#[poise::command(prefix_command, slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[rest = true]
    #[description = "The song you want to play"]
    query: String,
) -> Result<(), Error> {
    let channel_id = ctx
        .guild()
        .unwrap()
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let _vc = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.say("Not in a vc").await?;
            return Ok(());
        }
    };
    let manager = songbird::get(ctx.serenity_context()).await.unwrap();
    let (handle_lock, _) = manager
        .join(ctx.guild_id().unwrap(), channel_id.unwrap())
        .await;

    let mut handler = handle_lock.lock().await;

    let source = match query[..].starts_with("https") {
        true => ytdl(&query).await.unwrap(),
        false => ytdl_search(&query).await.unwrap(),
    };

    let title = source.metadata.title.as_ref().unwrap();
    let url = source.metadata.source_url.as_ref().unwrap();

    ctx.say(format!("Enqueued [{}]({})", title, url)).await?;

    handler.enqueue_source(source);
    Ok(())
}

use crate::{Context, Error, HttpKey};
use poise::CreateReply;
use songbird::{
    input::{AuxMetadata, Input, YoutubeDl},
    typemap::TypeMapKey,
};

pub struct TrackMetaKey;

impl TypeMapKey for TrackMetaKey {
    type Value = AuxMetadata;
}

/// Play music
#[poise::command(prefix_command, slash_command, aliases("p"))]
pub async fn play(
    ctx: Context<'_>,
    #[rest = true]
    #[description = "The music you want to play"]
    query: Option<String>,
) -> Result<(), Error> {
    let msg = ctx.say("Adding song...").await?;
    let guild_id = ctx.guild_id().expect("Could not retrieve guild_id");
    let channel_id = ctx
        .guild()
        .expect("Could not retrieve guild data")
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let vc = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.say("You must be in a voice channel first").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .expect("Guaranteed to exist in the typemap.")
    };

    let handler_lock = manager.join(guild_id, vc).await?;
    let mut handler = handler_lock.lock().await;

    let query = match query {
        Some(query) => query,
        None => {
            match handler.queue().current() {
                Some(track) => {
                    let _ = track.play();
                    msg.edit(
                        ctx,
                        CreateReply::default().content("Resumed playback.".to_string()),
                    )
                    .await?;
                }
                None => {
                    ctx.say("Must provide a url/song query").await?;
                    return Ok(());
                }
            };
            return Ok(());
        }
    };

    let src = if query.starts_with("http") {
        YoutubeDl::new_ytdl_like("./yt-dlp", http_client, query)
    } else {
        YoutubeDl::new_search_ytdl_like("./yt-dlp", http_client, query)
    };

    let mut input: Input = src.into();

    let metadata = input.aux_metadata().await?;

    let track_handle = handler.enqueue_input(input).await;

    track_handle
        .typemap()
        .write()
        .await
        .insert::<TrackMetaKey>(metadata.clone());

    msg.edit(
        ctx,
        CreateReply::default().content(format!(
            "Added `{}` to queue.",
            metadata.title.as_ref().expect("Invalid reference")
        )),
    )
    .await?;
    Ok(())
}

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
#[poise::command(prefix_command, slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[rest = true]
    #[description = "The music you want to play"]
    query: String,
) -> Result<(), Error> {
    let msg = ctx.say("Adding song...").await?;
    let guild_id = ctx.guild_id().unwrap();
    let channel_id = ctx
        .guild()
        .unwrap()
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

    let src = if query.starts_with("http") {
        YoutubeDl::new(http_client, query)
    } else {
        YoutubeDl::new_search(http_client, query)
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
            metadata.title.as_ref().unwrap()
        )),
    )
    .await?;
    Ok(())
}

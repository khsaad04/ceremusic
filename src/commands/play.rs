use crate::{Context, Error, HttpKey};
use songbird::input::{Compose, YoutubeDl};

/// Play music
#[poise::command(prefix_command, slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[rest = true]
    #[description = "The song you want to play"]
    query: String,
) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();
    let channel_id = ctx
        .guild()
        .unwrap()
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
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

    if let Ok(handler_lock) = manager.join(guild_id, connect_to).await {
        let mut handler = handler_lock.lock().await;

        let src = YoutubeDl::new(http_client, query);
        let metadata = src.clone().aux_metadata().await;

        let _ = handler.enqueue_input(src.clone().into()).await;

        if let Ok(data) = metadata {
            ctx.say(format!("Added `{}` to queue.", data.title.unwrap()))
                .await?;
        } else {
            ctx.say("Invalid input").await?;
        }
    }

    Ok(())
}

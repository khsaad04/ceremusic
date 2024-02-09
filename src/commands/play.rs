use crate::{Context, Error, HttpKey};
use songbird::input::YoutubeDl;

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
    let handle_lock = manager
        .join(ctx.guild_id().unwrap(), channel_id.unwrap())
        .await?;

    let mut handler = handle_lock.lock().await;
    let url = query;

    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .expect("Guaranteed to exist in the typemap.")
    };
    let src = YoutubeDl::new(http_client, url);

    let _ = handler.play_input(src.clone().into());

    Ok(())
}

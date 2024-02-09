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
    let url = query;
    let guild_id = ctx.guild_id().unwrap();

    let http_client = {
        let data = ctx.serenity_context().data.read().await;
        data.get::<HttpKey>()
            .cloned()
            .expect("Guaranteed to exist in the typemap.")
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialization.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let src = YoutubeDl::new(http_client, url);

        let _ = handler.play_input(src.clone().into());
        ctx.say("Playing song").await?;
    } else {
        ctx.say("Not in a voice channel").await?;
    }
    Ok(())
}

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
    let (guild_id, channel_id) = {
        let guild = ctx.guild().unwrap();
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        (guild.id, channel_id)
    };

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
        println!("{:?}", src);

        let _ = handler.play_input(src.clone().into());

        ctx.say("Playing song").await?;
    } else {
        ctx.say("Not in a vc!").await?;
    }

    Ok(())
}

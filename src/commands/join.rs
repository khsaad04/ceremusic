use crate::{Context, Error};

/// Join vc
#[poise::command(prefix_command, slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
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

    let _ = manager.join(guild_id, vc).await?;
    Ok(())
}

use crate::{Context, Error};

/// Show current queue
#[poise::command(prefix_command, slash_command)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild_id = ctx.guild_id().unwrap();

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let current_track = handler.queue().current_queue();
        ctx.say(format!("{:?}", current_track)).await?;
    }

    Ok(())
}

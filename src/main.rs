mod commands;

use poise::serenity_prelude as serenity;
use songbird::SerenityInit;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

use commands::{help::*, play::*};

#[tokio::main]
async fn main() {
    let discord_token = std::env::var("TOKEN_MUSIC").unwrap();

    let framework = poise::Framework::builder()
        .token(discord_token)
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("m!".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            commands: vec![help(), play()],
            ..Default::default()
        })
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .client_settings(|builder| builder.register_songbird())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });
    framework.run().await.unwrap();
}

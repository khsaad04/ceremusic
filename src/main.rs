mod commands;
mod utils;

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use reqwest::Client as HttpClient;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use songbird::{typemap::TypeMapKey, SerenityInit};

use commands::{dequeue, help, join, leave, now_playing, pause, play, queue, skip};
use utils::error::on_error;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

#[shuttle_runtime::main]
async fn poise(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
        && !std::process::Command::new("python")
            .arg("--version")
            .status()
            .expect("Python doesn't exist")
            .success()
    {
        panic!("failed to install dependencies")
    }

    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store.get("TOKEN").context("TOKEN' was not found")?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("m!".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            commands: vec![
                help::help(),
                play::play(),
                pause::pause(),
                now_playing::now_playing(),
                join::join(),
                leave::leave(),
                queue::queue(),
                dequeue::dequeue(),
                skip::skip(),
            ],
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(
        discord_token,
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
    )
    .framework(framework)
    .register_songbird()
    .type_map_insert::<HttpKey>(HttpClient::new())
    .await
    .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}

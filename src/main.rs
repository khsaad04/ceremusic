mod commands;

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use reqwest::Client as HttpClient;
use shuttle_secrets::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use songbird::typemap::TypeMapKey;
use songbird::SerenityInit;

use commands::{help::*, leave::*, play::*};

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

struct HttpKey;
impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

#[shuttle_runtime::main]
async fn poise(#[shuttle_secrets::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    if std::env::var("HOSTNAME")
        .unwrap_or_default()
        .contains("shuttle")
        && !std::process::Command::new("apt")
            .arg("install")
            .arg("-y")
            .arg("yt-dlp")
            .status()
            .expect("failed to run apt")
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
            commands: vec![help(), play(), leave()],
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

mod commands;
mod event_handler;
mod events;
mod util;

use crate::{commands::*, event_handler::Handler};
use default::default;
use dotenv::dotenv;
use poise::{
	serenity_prelude::{self as serenity, GatewayIntents, Ready},
	Framework, PrefixFrameworkOptions,
};
use std::env;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

type Error = Box<dyn std::error::Error + Send + Sync>;
type BlankResult = std::result::Result<(), Error>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

fn user_data_setup<'a>(
	_ctx: &'a serenity::Context,
	_ready: &'a Ready,
	_framework: &'a Framework<Data, Error>,
) -> Data {
	Data {}
}

#[tokio::main]
async fn main() -> BlankResult {
	// Logging
	let subscriber = FmtSubscriber::default();
	tracing::subscriber::set_global_default(subscriber)?;

	// Environment variables
	dotenv()?;
	info!("Getting DISCORD_API_KEY from .env");
	let discord_api_key = env::var("DISCORD_API_KEY")?;
	info!("Getting YOUTUBE_API_KEY from .env");
	let _youtube_api_key = env::var("YOUTUBE_API_KEY")?;

	// Bot
	let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
	let framework = Framework::builder()
		.client_settings(|c| c.event_handler(Handler))
		.options(poise::FrameworkOptions {
			commands: vec![doxx(), heheheha(), register(), help()],
			prefix_options: PrefixFrameworkOptions {
				prefix: Some("A!".into()),
				..default()
			},
			..default()
		})
		.token(discord_api_key)
		.intents(intents)
		.user_data_setup(move |ctx, ready, framework| {
			Box::pin(async move { Ok(user_data_setup(ctx, ready, framework)) })
		})
		.build()
		.await?;

	info!("Initialized");
	framework.start().await?;

	Ok(())
}

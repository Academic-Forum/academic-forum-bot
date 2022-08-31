mod commands;
mod event_handler;
mod util;

use crate::{commands::*, event_handler::Handler};
use default::default;
use dotenv::dotenv;
use poise::{
	serenity_prelude::{self as serenity, GatewayIntents, Ready},
	Framework, PrefixFrameworkOptions,
};
use std::env;

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
	dotenv()?;

	println!("Getting DISCORD_API_KEY from .env");
	let discord_api_key = env::var("DISCORD_API_KEY")?;
	println!("Getting YOUTUBE_API_KEY from .env");
	let _youtube_api_key = env::var("YOUTUBE_API_KEY")?;

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

	println!("Initialized");
	framework.start().await?;

	Ok(())
}

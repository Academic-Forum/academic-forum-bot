use default::default;
use dotenv::dotenv;
use lazy_static::lazy_static;
use poise::{
	serenity_prelude::{self as serenity, http::client, Ready},
	Framework, PrefixFrameworkOptions,
};
use std::env;

type Error = Box<dyn std::error::Error + Send + Sync>;
type BlankResult = std::result::Result<(), Error>;

struct Data {}

type Context<'a> = poise::Context<'a, Data, Error>;

lazy_static! {
	static ref HTTP: client::Http = client::Http::new(&env::var("DISCORD_API_KEY").unwrap());
}

/// Test command
#[poise::command(slash_command)]
async fn echo(ctx: Context<'_>, #[description = "Echo text"] echo: String) -> BlankResult {
	ctx.say(echo).await?;

	Ok(())
}

/// Leak Ved's IP
#[poise::command(slash_command)]
async fn doxx(ctx: Context<'_>) -> BlankResult {
	ctx.say("Ved's IP is `127.0.0.1`").await?;
	Ok(())
}

/// heheheha
#[poise::command(slash_command)]
async fn heheheha(ctx: Context<'_>) -> BlankResult {
	ctx.say("heheheha").await?;
	for i in 1..5 {
		ctx.channel_id()
			.send_message(&*HTTP, |m| m.content("heheheha"))
			.await?;
	}
	Ok(())
}

/// Register slash commands to discord
#[poise::command(slash_command, prefix_command)]
async fn register(ctx: Context<'_>) -> Result<(), Error> {
	ctx.say("Registering commands...").await?;

	let commands = &ctx.framework().options().commands;
	let create_commands = poise::builtins::create_application_commands(commands);
	serenity::Command::set_global_application_commands(ctx.discord(), |b| {
		*b = create_commands; // replace the given builder with the one prepared by poise
		b
	})
	.await?;

	ctx.say("Registered commands").await?;
	Ok(())
}

fn user_data_setup<'a>(
	ctx: &'a serenity::Context,
	ready: &'a Ready,
	framework: &'a Framework<Data, Error>,
) -> Data {
	Data {}
}

#[tokio::main]
async fn main() -> BlankResult {
	dotenv()?;

	println!("Getting DISCORD_API_KEY from .env");
	let discord_api_key = env::var("DISCORD_API_KEY")?;
	println!("Getting youtube API key from .env");
	let youtube_api_key = env::var("YOUTUBE_API_KEY")?;

	let framework = Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![echo(), doxx(), heheheha(), register()],
			prefix_options: PrefixFrameworkOptions {
				prefix: Some("A!".into()),
				case_insensitive_commands: true,
				..default()
			},
			..default()
		})
		.token(discord_api_key)
		.intents(
			serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
		)
		.user_data_setup(move |ctx, ready, framework| {
			Box::pin(async move { Ok(user_data_setup(ctx, ready, framework)) })
		});
	println!("Initialized");
	framework.run().await?;

	Ok(())
}

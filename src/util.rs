use crate::Context;
use lazy_static::lazy_static;
use poise::serenity_prelude::{http::client, Error, Message};
use std::env;

lazy_static! {
	static ref HTTP: client::Http = client::Http::new(&env::var("DISCORD_API_KEY").unwrap());
}

pub async fn send_message(ctx: Context<'_>, content: &str) -> Result<Message, Error> {
	ctx.channel_id()
		.send_message(&*HTTP, |m| m.content(content))
		.await
}

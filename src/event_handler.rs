use crate::events;
use async_trait::async_trait;
use poise::serenity_prelude::{Context, EventHandler, Message};
use tracing::error;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, message: Message) {
		let result = events::message(ctx, message).await;
		if let Err(err) = result {
			error!("{err}");
		}
	}
}

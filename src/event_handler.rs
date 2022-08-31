use async_trait::async_trait;
use poise::serenity_prelude::{Context, EventHandler, Message};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, message: Message) {
		if message.channel_id.0 != 824142410291478529 {
			return;
		}

		println!("{}", message.content);
		let number = match message.content.parse::<u32>() {
			Ok(number) => number,
			Err(_e) => {
				message.delete(&ctx.http).await.unwrap();
				// TODO DM user
				return;
			}
		};

		// These unwraps are safe
		let channel = message.channel(&ctx.http).await.unwrap().guild().unwrap();
		let prev_messages = channel
			.messages(&ctx.http, |g| g.before(message.id).limit(1))
			.await
			.unwrap();
		let prev_message = prev_messages.get(0).unwrap();
		let prev_number = match prev_message.content.parse::<u32>() {
			Ok(num) => num,
			Err(_e) => {
				prev_message.delete(&ctx.http).await.unwrap();
				message.delete(&ctx.http).await.unwrap();
				return;
			}
		};

		if number - 1 != prev_number {
			message.delete(&ctx.http).await.unwrap();
			// TODO DM user
		}
	}
}

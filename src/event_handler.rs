use async_trait::async_trait;
use poise::serenity_prelude::{Context, EventHandler, Message};
use std::{thread, time::Duration};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, message: Message) {
		if message.channel_id.0 != 824142410291478529 {
			return;
		}

		thread::sleep(Duration::from_secs(1));

		let number = match message.content.parse::<u32>() {
			Ok(number) => number,
			Err(_e) => {
				message.delete(&ctx.http).await.unwrap();
				println!(
					"Deleted message `{}` in #counting because it isn't a number.",
					message.content
				);
				// TODO DM user
				return;
			}
		};

		// These unwraps are safe
		let channel = message.channel(&ctx.http).await.unwrap().guild().unwrap();
		let prev_messages = channel
			.messages(&ctx.http, |g| g.before(message.id).limit(2))
			.await
			.unwrap();

		let prev_message = prev_messages.get(0).unwrap();
		let prev_number =
			match prev_message.content.parse::<u32>() {
				Ok(num) => num,
				Err(_e) => {
					prev_message.delete(&ctx.http).await.unwrap();
					message.delete(&ctx.http).await.unwrap();
					println!("Deleted the last two messages in #counting because they aren't valid numbers.");
					return;
				}
			};

		for prev_message in &prev_messages {
			if prev_message.author == message.author {
				message.delete(&ctx.http).await.unwrap();
				println!(
					"Deleted message `{}` in #counting because 2 other people haven't counted yet.",
					message.content
				);
				// TODO DM user
				return;
			}
		}

		if number - 1 != prev_number {
			message.delete(&ctx.http).await.unwrap();
			println!("Deleted message `{}` in #counting because it isn't 1 more than the previous number.", message.content);
			// TODO DM user
		}
	}
}

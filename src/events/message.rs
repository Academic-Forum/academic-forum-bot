use crate::BlankResult;
use poise::serenity_prelude::{Context, Message};
use std::{thread, time::Duration};
use tracing::info;

pub async fn message(ctx: Context, message: Message) -> BlankResult {
	if message.channel_id.0 != 824142410291478529 {
		return Ok(());
	}

	// Discord bugs out if a message is deleted too quickly after being posted, so wait a bit after the message send event
	thread::sleep(Duration::from_millis(100));

	let number = match message.content.parse::<u32>() {
		Ok(number) => number,
		Err(_e) => {
			message.delete(&ctx.http).await?;
			info!(
				"Deleted message `{}` in #counting because it isn't a number.",
				message.content
			);
			// TODO DM user
			return Ok(());
		}
	};

	// These unwraps are safe
	let channel = message.channel(&ctx.http).await?.guild().unwrap();
	let prev_messages = channel
		.messages(&ctx.http, |g| g.before(message.id).limit(2))
		.await?;

	let prev_message = prev_messages.first().unwrap();
	let prev_number = match prev_message.content.parse::<u32>() {
		Ok(num) => num,
		Err(_e) => {
			prev_message.delete(&ctx.http).await?;
			message.delete(&ctx.http).await?;
			info!("Deleted the last two messages in #counting because they aren't valid numbers.");
			return Ok(());
		}
	};

	for prev_message in &prev_messages {
		if prev_message.author == message.author {
			message.delete(&ctx.http).await?;
			info!(
				"Deleted message `{}` in #counting because 2 other people haven't counted yet.",
				message.content
			);
			// TODO DM user
			return Ok(());
		}
	}

	if number - 1 != prev_number {
		message.delete(&ctx.http).await?;
		info!(
			"Deleted message `{}` in #counting because it isn't 1 more than the previous number.",
			message.content
		);
		// TODO DM user
	}

	Ok(())
}

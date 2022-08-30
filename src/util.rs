use crate::Context;
use poise::{
	serenity_prelude::{Error, Message},
	ReplyHandle,
};

pub async fn append_edit(
	ctx: Context<'_>,
	message: ReplyHandle<'_>,
	content: &str,
) -> Result<(), Error> {
	let original_content = &message.message().await?.content;
	message
		.edit(ctx, |m| {
			m.content(format!("{}\n{}", original_content, content))
		})
		.await
}

pub async fn send_message(ctx: Context<'_>, content: &str) -> Result<Message, Error> {
	ctx.channel_id()
		.send_message(ctx.discord(), |m| m.content(content))
		.await
}

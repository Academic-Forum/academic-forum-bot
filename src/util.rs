use crate::Context;
use async_trait::async_trait;
use poise::{
	serenity_prelude::{Error, Message},
	ReplyHandle,
};

#[async_trait]
pub trait CustomContext {
	async fn append_edit(self, message: ReplyHandle<'_>, content: &str) -> Result<(), Error>;

	async fn say_message(&self, content: &str) -> Result<Message, Error>;
}

#[async_trait]
impl CustomContext for Context<'_> {
	async fn append_edit(self, message: ReplyHandle<'_>, content: &str) -> Result<(), Error> {
		let original_content = &message.message().await?.content;
		message
			.edit(self, |m| {
				m.content(format!("{}\n{}", original_content, content))
			})
			.await
	}

	async fn say_message(&self, content: &str) -> Result<Message, Error> {
		self.channel_id()
			.send_message(self.discord(), |m| m.content(content))
			.await
	}
}

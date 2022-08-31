use crate::{BlankResult, Context};

/// Leak Ved's IP
#[poise::command(slash_command)]
pub async fn doxx(ctx: Context<'_>) -> BlankResult {
	ctx.say("Ved's IP is `127.0.0.1`").await?;
	Ok(())
}

use crate::{util::CustomContext, BlankResult, Context};

/// heheheha
#[poise::command(slash_command)]
pub async fn heheheha(ctx: Context<'_>) -> BlankResult {
	ctx.say("heheheha").await?;
	for _i in 1..5 {
		ctx.say_message("heheheha").await?;
	}
	Ok(())
}

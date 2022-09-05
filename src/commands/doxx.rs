use crate::{BlankResult, Context};
use poise::serenity_prelude::User;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// Leak someone's IP using advanced Kali Linux hacking skills
#[poise::command(slash_command)]
pub async fn doxx(ctx: Context<'_>, #[description = "Person to doxx"] user: User) -> BlankResult {
	let ip = {
		let mut rng = ChaCha8Rng::seed_from_u64(user.id.0);
		format!(
			"{}.{}.{}.{}",
			rng.gen_range(0..255),
			rng.gen_range(0..255),
			rng.gen_range(0..255),
			rng.gen_range(0..255)
		)
	};

	ctx.send(|m| {
		m.content(format!("{}'s IP is {ip}", user))
			.allowed_mentions(|am| am.empty_users())
	})
	.await?;

	Ok(())
}

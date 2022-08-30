use crate::{BlankResult, Context, Data, Error};
use poise::Command;

/// List all commands
#[poise::command(slash_command)]
pub async fn help(
	ctx: Context<'_>,
	#[description = "A specific command to get help for"] command: Option<String>,
) -> BlankResult {
	let commands = &ctx.framework().options().commands;

	ctx.send(|m| {
		if let Some(command) = command {
			let matching_commands: Vec<&Command<Data, Error>> =
				commands.iter().filter(|c| c.name == command).collect();

			if let Some(command) = matching_commands.get(0) {
				m.embed(|e| {
					e.title(format!("Help for command `{}`", command.name))
						.description(format!(
							"**Description:**\n{}",
							command.description.as_ref().unwrap_or(&String::new())
						))
						.color(0xF8AA2A)
				});

				if command.parameters.len() > 0 {
					m.embed(|e| {
						e.title("Arguments").color(0xF8AA2A);
						for parameter in &command.parameters {
							e.field(
								&parameter.name,
								&parameter
									.description
									.as_ref()
									.unwrap_or(&"No description".to_string()),
								false,
							);
						}
						e
					});
				};
			} else {
				m.embed(|e| {
					e.description(format!("The command `{command}` doesn't exist."))
						.color(0xFF0000)
				});
			}
		} else {
			m.embed(|e| {
				e.title("Help")
					.description("List of all commands")
					.color(0xF8AA2A);
				for command in commands {
					e.field(
						&command.name,
						command.description.as_ref().unwrap_or(&String::new()),
						false,
					);
				}
				e
			});
		}
		m
	})
	.await?;
	Ok(())
}

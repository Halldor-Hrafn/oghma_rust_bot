use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

#[allow(dead_code)]
pub fn run(_command: &ApplicationCommandInteraction) -> String {
    "Hey, I'm alive!".to_string()
}

#[allow(dead_code)]
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
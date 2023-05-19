use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandData;

pub fn run(_data: &CommandData) -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
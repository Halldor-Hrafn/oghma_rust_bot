use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandData;

pub fn run(data: &CommandData) -> String {
    let options = &data.options;
    println!("{:#?}", &options);

    "TODO".to_string()
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("create")
        .description("Create a new task")
        .create_option(|option| {
            option
                .name("name")
                .description("Name of the task")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("description")
                .description("Description of the task")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("priority")
                .description("Priority of the task")
                .kind(CommandOptionType::Integer)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("assignee")
                .description("Assignee of the task")
                .kind(CommandOptionType::User)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("project")
                .description("Project of the task")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("status")
                .description("Status of the task")
                .kind(CommandOptionType::String)
                .required(false)
        })
}
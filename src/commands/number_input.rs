use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

pub fn run(command: &ApplicationCommandInteraction) -> String {
    let options = &command.data.options;
    println!("{:#?}", &options);

    let int = &options[0].value.as_ref().unwrap().as_u64().unwrap();
    let number = &options[1].value.as_ref().unwrap().as_f64().unwrap();

    let result = int.to_owned() as f64 + number;

    result.to_string()
}

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("numberinput")
        .description("Test command for number input")
        .create_option(|option| {
            option
                .name("int")
                .description("An integer from 5 to 10")
                .kind(CommandOptionType::Integer)
                .min_int_value(5)
                .max_int_value(10)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("number")
                .description("A float from -3.3 to 234.5")
                .kind(CommandOptionType::Number)
                .min_number_value(-3.3)
                .max_number_value(234.5)
                .required(true)
        })
}
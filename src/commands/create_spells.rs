use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandData;

use postgrest::Postgrest;

use dotenv::dotenv;

pub fn run(data: &CommandData) -> String {
    let options = &data.options;
    dotenv().ok();
    let name = options
        .iter()
        .find(|option| option.name == "name")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let cast_time = options
        .iter()
        .find(|option| option.name == "cast_time")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let range = options
        .iter()
        .find(|option| option.name == "range")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let components = options
        .iter()
        .find(|option| option.name == "components")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let duration = options
        .iter()
        .find(|option| option.name == "duration")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let school = options
        .iter()
        .find(|option| option.name == "school")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let attack_save = options
        .iter()
        .find(|option| option.name == "attack_save")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let damage_effect = options
        .iter()
        .find(|option| option.name == "damage_effect")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();

    println!("name: {:?}", name);
    println!("cast_time: {:?}", cast_time);
    println!("range: {:?}", range);
    println!("components: {:?}", components);
    println!("duration: {:?}", duration);
    println!("school: {:?}", school);
    println!("attack_save: {:?}", attack_save);
    println!("damage_effect: {:?}", damage_effect);

    let client = Postgrest::new(std::env::var("DATABASE_URL").unwrap().as_str())
        .insert_header("apikey", std::env::var("SUPABASE_PUBLIC_KEY").unwrap().as_str());

    "TODO: ACTUALLY GET SUPABASE WORKING SO YOU CAN START STORING THE CREATED SPELLS IN A DATABASE INSTEAD OF JUST NOT USING THE 200 LINES OF CODE YOU WROTE YOU FUCKING TWAT".to_string()
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("create_spells")
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
                .name("cast_time")
                .description("Description of the task")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("range")
                .description("Priority of the task")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("components")
                .description("Assignee of the task")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("duration")
                .description("Project of the task")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("school")
                .description("Status of the task")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("attack_save")
                .description("Status of the task")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("damage_effect")
                .description("Status of the task")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
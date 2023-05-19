use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandData;

use postgrest::Postgrest;

use postgres::{Client, NoTls};

use dotenv::dotenv;

pub async fn run(data: &CommandData) -> String {
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

    // println!("name: {:?}", name);
    // println!("cast_time: {:?}", cast_time);
    // println!("range: {:?}", range);
    // println!("components: {:?}", components);
    // println!("duration: {:?}", duration);
    // println!("school: {:?}", school);
    // println!("attack_save: {:?}", attack_save);
    // println!("damage_effect: {:?}", damage_effect);

    let insert_data = format!(
        r#"[{{"name": "{}","cast_time": "{}","range": "{}","components": "{}","duration": "{}","school": "{}","attack_save": "{}","damage_effect": "{}"}}]"#,
        name, cast_time, range, components, duration, school, attack_save, damage_effect
    );

    // println!("insert_data: {:#?}", insert_data);

    let client = Postgrest::new(std::env::var("SUPABASE_URL").unwrap().as_str())
        .insert_header("apikey", std::env::var("SUPABASE_PUBLIC_KEY").unwrap().as_str());

    let response = client
        .from("spells")
        .insert("[{\"name\": \"test\"}]")
        .execute()
        .await;

    let body = response.unwrap().text().await.unwrap();

    println!("body: {:#?}", body);
    

// std::env::var("SUPABASE_URL").unwrap().as_ref()
    // let mut client = Client::connect("https://qjfsewhwuoifymwqsazn.supabase.co/rest/v1", NoTls)?;

    // let stmt = "INSERT INTO spells (name, cast_time, range, components, duration, school, attack_save, damage_effect) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

    // client.execute(stmt, &[&name, &cast_time, &range, &components, &duration, &school, &attack_save, &damage_effect])?;

    // let insert_result = insert_into_postgres(name.to_string(), cast_time.to_string(), range.to_string(), components.to_string(), duration.to_string(), school.to_string(), attack_save.to_string(), damage_effect.to_string());

    // match insert_result {
    //     Ok(_) => return "Successfully inserted into Supabase".to_string(),
    //     Err(e) => return format!("Error inserting into Supabase: {}", e.to_string())
    // }

    "I hope this fucking works this time".to_string()
}

pub fn insert_into_postgres(
    name: String,
    cast_time: String, 
    range: String, 
    components: String,
    duration: String,
    school: String,
    attack_save: String,
    damage_effect: String
) -> Result<(), postgres::error::Error> {
    let mut client = Client::connect(std::env::var("SUPABASE_URL").unwrap().as_ref(), NoTls)?;

    let stmt = "INSERT INTO spells (name, cast_time, range, components, duration, school, attack_save, damage_effect) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)";

    client.execute(stmt, &[&name, &cast_time, &range, &components, &duration, &school, &attack_save, &damage_effect])?;

    Ok(())
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
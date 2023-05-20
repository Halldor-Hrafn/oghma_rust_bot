use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandData;
use serenity::http::Http;

use serde_json::json;

use postgrest::Postgrest;

use dotenv::dotenv;

pub async fn run(data: &CommandData) -> String {
    dotenv().ok();

    let guild = data.guild_id.unwrap();

    let application_id = std::env::var("APPLICATION_ID")
    .expect("APPLICATION_ID environment variable not set")
    .parse::<u64>()
    .expect("Failed to parse APPLICATION_ID as u64");

    let http = Http::new_with_application_id(&std::env::var("DISCORD_TOKEN").unwrap(), application_id);
    let member_count = guild.members(http, None, None).await.unwrap().len() as u64;

    let options = &data.options;

    let name = options
        .iter()
        .find(|option| option.name == "name")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let level = options
        .iter()
        .find(|option| option.name == "level")
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

    let insert_data_1 = json!({
        "name": name,
        "level": level,
        "cast_time": cast_time,
        "range": range,
        "components": components,
        "duration": duration,
        "school": school,
        "attack_save": attack_save,
        "damage_effect": damage_effect,
        "guild_id": guild
    }).to_string();

    let postgrest_client = Postgrest::new(std::env::var("SUPABASE_URL").unwrap().as_str())
        .insert_header("apikey", std::env::var("SUPABASE_PUBLIC_KEY").unwrap().as_str());

    let resp_1 = postgrest_client
        .from("spells")
        .insert(insert_data_1)
        .execute()
        .await;

    let body_1 = resp_1.unwrap().text().await.unwrap();

    println!("body: {:#?}", body_1);

    let insert_data_2 = json!({
        "guild_id": guild,
        "member_count": member_count,
    }).to_string();

    let resp_2 = postgrest_client
        .from("guilds")
        .upsert(insert_data_2)
        .execute()
        .await;

    let body_2 = resp_2.unwrap().text().await.unwrap();

    println!("body: {:#?}", body_2);

    let response = format!("Spell named {} created with a cast time of {}\n\
        a range of {}\ncomponents of {}\nduration of {}\n\
        in the school of {}\n with a attack_save throw of {}\n\
        and with damage/effect of {}", name, cast_time, range, 
        components, duration, school, attack_save, damage_effect
    );

    response
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("create_spells")
        .description("Create a new spell")
        .create_option(|option| {
            option
                .name("name")
                .description("Name of the spell")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("level")
                .description("The spells' level")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("cast_time")
                .description("Cast time of the spell")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("range")
                .description("Range of the spell")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("components")
                .description("Components of the spell")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("duration")
                .description("Duration of the spells' effects")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("school")
                .description("School of the spell")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("attack_save")
                .description("The attack/save modifier of the spell")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("damage_effect")
                .description("The damage/effect of the spell")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
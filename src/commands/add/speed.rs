use serenity::builder;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use serde_json::json;

use postgrest::Postgrest;

use dotenv::dotenv;

use serde::{Serialize, Deserialize};

use colorized::*;

#[derive(Serialize, Deserialize, Debug)]
struct MonsterId {
    id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    id_monster: i64,
    id_speed: i64,
    range: String,
}

pub async fn run(command: &ApplicationCommandInteraction) -> String {
    let guild_id = command.guild_id.unwrap().to_string();
    let user_id = command.user.id.to_string();

    let options = &command.data.options;

    let name = options
        .iter()
        .find(|option| option.name == "name")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let speed = options
        .iter()
        .find(|option| option.name == "speed")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_i64()
        .unwrap();
    let type_ = options
        .iter()
        .find(|option| option.name == "type")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();

    let postgrest_client = Postgrest::new(std::env::var("POSTGREST_URL")
        .expect(colorize_this("Expected a URL in the environment", Colors::RedFg).as_str()).as_str())
        .insert_header("apikey", std::env::var("POSTGREST_PUBLIC_KEY")
        .expect(colorize_this("Expected an API key in the environment", Colors::RedFg).as_str()).as_str());

    let resp_1 = postgrest_client
        .from("monsters")
        .select("id")
        .eq("name", name)
        .eq("guild_id", guild_id.as_str())
        .eq("user_id", user_id.as_str())
        .execute()
        .await;

    let monster_id = resp_1.unwrap().text().await.unwrap();

    let monster_id: Vec<MonsterId> = serde_json::from_str(monster_id.as_str()).unwrap();

    let speed_id: i64 = type_.parse().unwrap();

    let data = Data {
        id_monster: monster_id[0].id,
        id_speed: speed_id,
        range: speed.to_string(),
    };

    let resp_2 = postgrest_client
        .from("monster_speeds")
        .insert(json!(data).to_string())
        .execute()
        .await;

    let _body_2 = resp_2.unwrap().text().await.unwrap();

    // println!("{}", _body_2);
    // TODO: Fix this shit with the fact that the variable type_ is a number and not a string so it might display shit like "Gave a a speed of 50 2"
    let response = format!("Gave {} a speed of {} {}", name, speed, type_).to_string();

    response
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("add_speed")
        .description("Add a speed to a monster")
        .dm_permission(false)
        .create_option(|option| {
            option
                .name("name")
                .description("The name of the monster")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("speed")
                .description("The speed of the monster")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("type")
                .description("The type of speed")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("walk", 1)
                .add_string_choice("fly", 2)
                .add_string_choice("swim", 3)
                .add_string_choice("climb", 4)
                .add_string_choice("burrow", 5)
        })
}
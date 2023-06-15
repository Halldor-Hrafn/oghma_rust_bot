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

#[allow(dead_code)]
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
    let proficiency = options
        .iter()
        .find(|option| option.name == "proficiency")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let value = options
        .iter()
        .find(|option| option.name == "value")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_i64()
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

    let monster_id = monster_id[0].id;

    let resp_2 = postgrest_client
        .from("proficiencies")
        .insert(json!({
            "id_monster": monster_id,
            "proficiency": proficiency,
            "value": value,
        }).to_string())
        .execute()
        .await;

    "TODO".to_string()
}

#[allow(dead_code)]
pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("add_proficiency")
        .description("Add a proficiency to a monster")
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
                .name("proficiency")
                .description("The proficiency to add")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("value")
                .description("The value of the proficiency")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
}

use serenity::builder;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use postgrest::Postgrest;

use serde::{Serialize, Deserialize};

use colorized::*;

#[derive(Serialize, Deserialize)]
struct MonsterData {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    command: String,
    user_id: String,
    monsters: Vec<MonsterData>
}

#[allow(dead_code)]
pub async fn run(command: &ApplicationCommandInteraction) -> String {
    let guild_id = command.guild_id.unwrap().to_string();
    let user_id = command.user.id.to_string();

    let user = if let Some(option) = command.data.options.iter().find(|option| option.name == "user") {
        option.value.as_ref().unwrap().as_str().unwrap()
    } else {
        &user_id
    };

    let postgrest_client = Postgrest::new(std::env::var("POSTGREST_URL")
        .expect(colorize_this("Expected a URL in the environment", Colors::RedFg).as_str()).as_str())
        .insert_header("apikey", std::env::var("POSTGREST_PUBLIC_KEY")
        .expect(colorize_this("Expected an API key in the environment", Colors::RedFg).as_str()).as_str());

    let resp = postgrest_client
        .from("monsters")
        .select("name")
        .eq("guild_id", guild_id.as_str())
        .eq("user_id", user)
        .execute()
        .await;

    let body = resp.unwrap().text().await.unwrap();

    colorize_println(format!("body: {:#?}", body), Colors::BrightYellowFg);

    let deserialized_monsters_vec: Vec<MonsterData> = serde_json::from_str(&body).unwrap();

    let data = Data {
        command: "command_list_monsters".to_string(),
        user_id: user.to_string(),
        monsters: deserialized_monsters_vec,
    };

    let json = serde_json::to_string(&data).unwrap();

    json
}

#[allow(dead_code)]
pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("list_monsters")
        .description("List monsters")
        .dm_permission(false)
        .create_option(|option| {
            option
                .name("user")
                .description("User to list monsters for")
                .kind(CommandOptionType::User)
                .required(false)
        })
}

pub fn create_embed(content: &str) -> builder::CreateEmbed {
    let data: Data = serde_json::from_str(content).unwrap();

    CreateEmbed::default()
        .title("Monsters")
        .description(format!("Monsters created by <@{}>", data.user_id))
        .fields(data.monsters.iter().map(|monster| {
            (
                monster.name.clone(),
                "".to_owned(), 
                false
            )
        }).collect::<Vec<(String, String, bool)>>()).to_owned()
}
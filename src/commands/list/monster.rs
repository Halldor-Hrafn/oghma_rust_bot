use serenity::builder;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use postgrest::Postgrest;

use serde::{Serialize, Deserialize};

use dotenv::dotenv;

use colorized::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MonsterData {
    id: i64,
    name: String,
    size: String,
    type_: String,
    alignment: String,
    armor_class: String,
    hit_points: i32,
    challenge_rating: String,
    guild_id: String,
    user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SpeedData {
    range: String,
    speeds: SpeedName
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SpeedName {
    name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CompleteData {
    monster_data: MonsterData,
    speed_data: Option<Vec<SpeedData>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Data {
    command: String,
    monsters: CompleteData
}

pub async fn run(command: &ApplicationCommandInteraction) -> String {
    dotenv().ok();

    let guild_id = command.guild_id.unwrap().to_string();
    let user_id = command.user.id.to_string();

    let user = if let Some(option) = command.data.options.iter().find(|option| option.name == "user") {
        option.value.as_ref().unwrap().as_str().unwrap()
    } else {
        &user_id
    };

    let name = command.data.options.iter().find(|option| option.name == "name").unwrap().value.as_ref().unwrap().as_str().unwrap();

    let postgrest_client = Postgrest::new(std::env::var("POSTGREST_URL")
        .expect(colorize_this("Expected a URL in the environment", Colors::RedFg).as_str()).as_str())
        .insert_header("apikey", std::env::var("POSTGREST_PUBLIC_KEY")
        .expect(colorize_this("Expected an API key in the environment", Colors::RedFg).as_str()).as_str());

    let monster_resp = postgrest_client
        .from("monsters")
        .select("*")
        .eq("guild_id", guild_id.as_str())
        .eq("user_id", user)
        .eq("name", name)
        .execute()
        .await;

    let monster_body = monster_resp.unwrap().text().await.unwrap();

    let deserialized_monster_vec: Vec<MonsterData> = serde_json::from_str(&monster_body).unwrap();

    println!("deserialized_monster_vec: {:#?}", deserialized_monster_vec);

    let monster_data = deserialized_monster_vec.first().unwrap().to_owned();

    let speed_resp = postgrest_client
        .from("monster_speeds")
        .select(
            "range, speeds(name)"
        )
        .eq("id_monster", &monster_data.id.to_string().as_str())
        .execute()
        .await;

    let speed_body = speed_resp.unwrap().text().await.unwrap();

    println!("speed_body: {:#?}", speed_body);

    let deserialized_speed_vec: Vec<SpeedData> = if speed_body == "[]" {
        Vec::new()
    } else {
        serde_json::from_str(&speed_body).unwrap()
    };

    println!("deserialized_speed_vec: {:#?}", &deserialized_speed_vec);

    let complete_data = CompleteData {
        monster_data: monster_data.clone(),
        speed_data: if deserialized_speed_vec.is_empty() {
            None
        } else {
            Some(deserialized_speed_vec)
        },
    };

    let data = Data {
        command: "command_list_monster".to_string(),
        monsters: complete_data,
    };

    let json = serde_json::to_string(&data).unwrap();

    json
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("list_monster")
        .description("List a monster")
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
                .name("user")
                .description("The user to list monsters for")
                .kind(CommandOptionType::User)
                .required(false)
        })
}

pub fn create_embed<'a>(content: &str) -> builder::CreateEmbed {
    let data: Data = serde_json::from_str(content).unwrap();

    let monster = data.monsters.monster_data.clone();

    CreateEmbed::default()
        .title(format!("*{}*", monster.name))
        .field("size", monster.size, false)
        .field("type", monster.type_, false)
        .field("alignment", monster.alignment, false)
        .field("armor class", monster.armor_class, false)
        .field("hit points", monster.hit_points, false)
        .field("challenge rating", monster.challenge_rating, false)
        .to_owned()
}

pub fn create_speeds_embed(content: &str) -> builder::CreateEmbed {
    let data: Data = serde_json::from_str(content).unwrap();

    match data.monsters.speed_data.clone() {
        Some(speeds) => {
            return CreateEmbed::default()
                .title("Speeds")
                .fields(speeds.iter().map(|speed| {
                    (
                        speed.speeds.name.clone(),
                        speed.range.clone(),
                        false
                    )
                }).collect::<Vec<(String, String, bool)>>())
                .to_owned()
        },
        None => {
            return CreateEmbed::default()
                .title("Speeds")
                .description("No speeds found, you can give this monster speed by using /add_speed")
                .to_owned()
        }
    }
}
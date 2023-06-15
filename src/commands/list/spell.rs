use serenity::builder;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use postgrest::Postgrest;

use serde::{Serialize, Deserialize};

use colorized::*;

#[derive(Serialize, Deserialize)]
struct SpellData {
    name: String,
    level: String,
    cast_time: String,
    range: String,
    components: String,
    duration: String,
    school: String,
    attack_save: String,
    damage_effect: String,
    guild_id: String,
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    command: String,
    spells: Vec<SpellData>
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

    let name = command.data.options
        .iter()
        .find(|option| option.name == "name")
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

    let resp = postgrest_client
        .from("spells")
        .select("*")
        .eq("guild_id", guild_id.as_str())
        .eq("user_id", user)
        .eq("name", name)
        .execute()
        .await;

    let body = resp.unwrap().text().await.unwrap();

    colorize_println(format!("body: {:#?}", body), Colors::BrightYellowFg);

    let deserialized_spells_vec: Vec<SpellData> = serde_json::from_str(&body).unwrap();

    let data = Data {
        command: "command_list_spell".to_string(),
        spells: deserialized_spells_vec,
    };

    let data_json = serde_json::to_string(&data).unwrap();

    data_json
}

#[allow(dead_code)]
pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("list_spell")
        .description("Lists one spell")
        .dm_permission(false)
        .create_option(|option| {
            option
                .name("name")
                .description("The name for the spell")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("user")
                .description("The user to list spells for")
                .kind(CommandOptionType::User)
                .required(false)
        })
}

pub fn create_embed(data: &str) -> builder::CreateEmbed {
    let data: Data = serde_json::from_str(data).unwrap();
    let spell = &data.spells[0];

    CreateEmbed::default()
        .title(format!("*{}*", spell.name))
        .field("Name", spell.name.as_str(), false)
        .field("Level", spell.level.as_str(), false)
        .field("Cast Time", spell.cast_time.as_str(), false)
        .field("Range", spell.range.as_str(), false)
        .field("Components", spell.components.as_str(), false)
        .field("Duration", spell.duration.as_str(), false)
        .field("School", spell.school.as_str(), false)
        .field("Attack/Save", spell.attack_save.as_str(), false)
        .field("Damage/Effect", spell.damage_effect.as_str(), false)
        .to_owned()
}
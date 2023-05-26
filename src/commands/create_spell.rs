use serenity::builder;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandData;
use serenity::http::Http;

use serde_json::json;

use postgrest::Postgrest;

use dotenv::dotenv;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct SpellData {
    command: String,
    name: String,
    level: String,
    cast_time: String,
    range: String,
    components: String,
    duration: String,
    school: String,
    attack_save: String,
    damage_effect: String,
}

pub async fn run(data: &CommandData) -> String {
    // println!("data: {:#?}", data);
    dotenv().ok();

    let guild = data.guild_id.unwrap();

    let application_id = std::env::var("APPLICATION_ID")
    .expect("APPLICATION_ID environment variable not set")
    .parse::<u64>()
    .expect("Failed to parse APPLICATION_ID as u64");

    let http = Http::new_with_application_id(&std::env::var("DISCORD_TOKEN").unwrap(), application_id);
    let _member_count = guild.members(http, None, None).await.unwrap().len() as u64;

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

    let spell_data = SpellData {
        command: "command_create_spells".to_owned(),
        name: name.to_owned(),
        level: level.to_owned(),
        cast_time: cast_time.to_owned(),
        range: range.to_owned(),
        components: components.to_owned(),
        duration: duration.to_owned(),
        school: school.to_owned(),
        attack_save: attack_save.to_owned(),
        damage_effect: damage_effect.to_owned(),
    };

    let postgrest_client = Postgrest::new(std::env::var("SUPABASE_URL").unwrap().as_str())
        .insert_header("apikey", std::env::var("SUPABASE_PUBLIC_KEY").unwrap().as_str());

    let resp_1 = postgrest_client
        .from("spells")
        .insert(json!(spell_data).to_string())
        .execute()
        .await;

    let _body_1 = resp_1.unwrap().text().await.unwrap();

    let spell_data_json = serde_json::to_string(&spell_data).unwrap();

    spell_data_json
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("create_spell")
        .description("Creates a new spell.")
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

pub fn create_spell_embed(data: &str) -> builder::CreateEmbed {
    let spell_data: SpellData = serde_json::from_str(data).unwrap();

    let embed = CreateEmbed::default()
        .title("Spell Created")
        .description(format!("Spell {} has been created!", spell_data.name))
        .field("Name", spell_data.name, true)
        .field("Level", spell_data.level, true)
        .field("Cast Time", spell_data.cast_time, true)
        .field("Range", spell_data.range, true)
        .field("Components", spell_data.components, true)
        .field("Duration", spell_data.duration, true)
        .field("School", spell_data.school, true)
        .field("Attack/Save", spell_data.attack_save, true)
        .field("Damage/Effect", spell_data.damage_effect, true)
        .to_owned();

    embed
}

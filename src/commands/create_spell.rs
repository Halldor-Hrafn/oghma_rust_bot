use serenity::builder;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use serde_json::json;

use postgrest::Postgrest;

use dotenv::dotenv;

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
    spells: SpellData
}

pub async fn run(command: &ApplicationCommandInteraction) -> String {
    // println!("data: {:#?}", command.data);
    dotenv().ok();

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
        name: name.to_owned(),
        level: level.to_owned(),
        cast_time: cast_time.to_owned(),
        range: range.to_owned(),
        components: components.to_owned(),
        duration: duration.to_owned(),
        school: school.to_owned(),
        attack_save: attack_save.to_owned(),
        damage_effect: damage_effect.to_owned(),
        guild_id: guild_id.to_owned(),
        user_id: user_id.to_owned(),
    };

    let postgrest_client = Postgrest::new(std::env::var("SUPABASE_URL")
        .expect(colorize_this("Expected a URL in the environment", Colors::RedFg).as_str()).as_str())
        .insert_header("apikey", std::env::var("SUPABASE_PUBLIC_KEY")
        .expect(colorize_this("Expected an API key in the environment", Colors::RedFg).as_str()).as_str());

    let resp = postgrest_client
        .from("spells")
        .insert(json!(spell_data).to_string())
        .execute()
        .await;

    let _body = resp.unwrap().text().await.unwrap();

    colorize_println(format!("body: {:#?}", _body), Colors::BrightYellowFg);

    let data = Data {
        command: "command_create_spells".to_owned(),
        spells: spell_data,
    };

    let data_json = serde_json::to_string(&data).unwrap();

    data_json
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

// pub fn create_spell_embed(data: &str) -> builder::CreateEmbed {
//     let data: Data = serde_json::from_str(data).unwrap();

//     let embed = CreateEmbed::default()
//         .title("Spell created")
//         .description(format!("Spell {} has been created!", data.spells.name))
//         .field("Name", data.spells.name, true)
//         .field("Level", data.spells.level, true)
//         .field("Cast Time", data.spells.cast_time, true)
//         .field("Range", data.spells.range, true)
//         .field("Components", data.spells.components, true)
//         .field("Duration", data.spells.duration, true)
//         .field("School", data.spells.school, true)
//         .field("Attack/Save", data.spells.attack_save, true)
//         .field("Damage/Effect", data.spells.damage_effect, true)
//         .to_owned();

//     embed
// }

pub fn create_spell_embed(data: &str) -> builder::CreateEmbed {
    let data: Data = serde_json::from_str(data).unwrap();
    let spell = &data.spells;

    CreateEmbed::default()
        .title("Spell created")
        .description(format!("Spell {} has been created!", spell.name))
        .field("Name", spell.name.as_str(), true)
        .field("Level", spell.level.as_str(), true)
        .field("Cast Time", spell.cast_time.as_str(), true)
        .field("Range", spell.range.as_str(), true)
        .field("Components", spell.components.as_str(), true)
        .field("Duration", spell.duration.as_str(), true)
        .field("School", spell.school.as_str(), true)
        .field("Attack/Save", spell.attack_save.as_str(), true)
        .field("Damage/Effect", spell.damage_effect.as_str(), true)
        .to_owned()
}

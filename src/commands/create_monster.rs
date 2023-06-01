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
struct ArmorClass {
    type_: String,
    value: i32,
}

#[derive(Serialize, Deserialize)]
struct SpeedData {
    type_: String,
    value: i32,
}

#[derive(Serialize, Deserialize)]
struct AbilityScore {
    name: String,
    value: i32,
}

#[derive(Serialize, Deserialize)]
struct Proficiency {
    name: String,
    value: i32,
}

#[derive(Serialize, Deserialize)]
struct SpecialAbilities {
    name: String,
    desc: String,
}

#[derive(Serialize, Deserialize)]
struct MonsterData {
    name: String,
    size: String,
    type_: String,
    alignment: String,
    armor_class: ArmorClass,
    hit_points: i32,
    speed: Vec<SpeedData>,
    ability_scores: Vec<AbilityScore>,
    proficiencies: Vec<Proficiency>,
    damage_vulnerabilities: Vec<String>,
    damage_resistances: Vec<String>,
    damage_immunities: Vec<String>,
    condition_immunities: Vec<String>,
    senses: Vec<String>,
    languages: Vec<String>,
    challenge_rating: String,
    special_abilities: Vec<SpecialAbilities>,
    actions: Vec<String>,
}

pub async fn run(command: &ApplicationCommandInteraction) -> String {
    colorize_println(format!("data: {:#?}", command.data), Colors::BlueFg);

    "TODO".to_string()
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("create_monster")
        .description("Creates a new monster")
        .create_option(|option| {
            option
                .name("name")
                .description("The name of the monster")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("size")
                .description("The size of the monster")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("Tiny", "Tiny")
                .add_string_choice("Small", "Small")
                .add_string_choice("Medium", "Medium")
                .add_string_choice("Large", "Large")
                .add_string_choice("Huge", "Huge")
                .add_string_choice("Gargantuan", "Gargantuan")
        })
        .create_option(|option| {
            option
                .name("type")
                .description("The type of the monster")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("Aberration", "Aberration")
                .add_string_choice("Beast", "Beast")
                .add_string_choice("Celestial", "Celestial")
                .add_string_choice("Construct", "Construct")
                .add_string_choice("Dragon", "Dragon")
                .add_string_choice("Elemental", "Elemental")
                .add_string_choice("Fey", "Fey")
                .add_string_choice("Fiend", "Fiend")
                .add_string_choice("Giant", "Giant")
                .add_string_choice("Humanoid", "Humanoid")
                .add_string_choice("Monstrosity", "Monstrosity")
                .add_string_choice("Ooze", "Ooze")
                .add_string_choice("Plant", "Plant")
                .add_string_choice("Undead", "Undead")
        })
        .create_option(|option| {
            option
                .name("alignment")
                .description("The alignment of the monster")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("Lawful Good", "Lawful Good")
                .add_string_choice("Neutral Good", "Neutral Good")
                .add_string_choice("Chaotic Good", "Chaotic Good")
                .add_string_choice("Lawful Neutral", "Lawful Neutral")
                .add_string_choice("Neutral", "Neutral")
                .add_string_choice("Chaotic Neutral", "Chaotic Neutral")
                .add_string_choice("Lawful Evil", "Lawful Evil")
                .add_string_choice("Neutral Evil", "Neutral Evil")
                .add_string_choice("Chaotic Evil", "Chaotic Evil")
        })
        .create_option(|option| {
            option
                .name("armor_class")
                .description("The armor class of the monster")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("armor_type")
                .description("The armor type of the monster")
                .kind(CommandOptionType::String)
                .required(false)
                .add_string_choice("Natural Armor", "Natural Armor")
                .add_string_choice("Armor", "Armor")
                .add_string_choice("Shield", "Shield")
        })
        .create_option(|option| {
            option
                .name("hit_points")
                .description("The hit points of the monster")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("speed")
                .description("The speed of the monster")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("strength")
                .description("The strength of the monster")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("dexterity")
                .description("The dexterity of the monster")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("constitution")
                .description("The constitution of the monster")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("intelligence")
                .description("The intelligence of the monster")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("wisdom")
                .description("The wisdom of the monster")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("charisma")
                .description("The charisma of the monster")
                .kind(CommandOptionType::Integer)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("challenge_rating")
                .description("The challenge rating of the monster")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("special_abilities")
                .description("The special abilities of the monster")
                .kind(CommandOptionType::String)
                .required(false)
        })
        .create_option(|option| {
            option
                .name("actions")
                .description("The actions of the monster")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("legendary_actions")
                .description("The legendary actions of the monster")
                .kind(CommandOptionType::String)
                .required(false)
        })
        
}

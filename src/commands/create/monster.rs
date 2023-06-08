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
struct MonsterData {
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

#[derive(Serialize, Deserialize)]
struct Data {
    command: String,
    monsters: MonsterData
}

pub async fn run(command: &ApplicationCommandInteraction) -> String {
    colorize_println(format!("data: {:#?}", command.data), Colors::BlueFg);
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
    let size = options
        .iter()
        .find(|option| option.name == "size")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
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
    let alignment = options
        .iter()
        .find(|option| option.name == "alignment")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let armor_class = options
        .iter()
        .find(|option| option.name == "armor_class")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let hit_points = options
        .iter()
        .find(|option| option.name == "hit_points")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let challenge_rating = options
        .iter()
        .find(|option| option.name == "challenge_rating")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();

    let monster_data = MonsterData {
        name: name.to_string(),
        size: size.to_string(),
        type_: type_.to_string(),
        alignment: alignment.to_string(),
        armor_class: armor_class.to_string(),
        hit_points: hit_points.parse::<i32>().unwrap(),
        challenge_rating: challenge_rating.to_string(),
        guild_id: guild_id.to_string(),
        user_id: user_id.to_string(),
    };

    let postgrest_client = Postgrest::new(std::env::var("POSTGREST_URL")
        .expect(colorize_this("Expected a URL in the environment", Colors::RedFg).as_str()).as_str())
        .insert_header("apikey", std::env::var("POSTGREST_PUBLIC_KEY")
        .expect(colorize_this("Expected an API key in the environment", Colors::RedFg).as_str()).as_str());

    let resp_1 = postgrest_client
        .from("monsters")
        .select("*")
        .eq("name", &monster_data.name)
        .eq("guild_id", &guild_id)
        .eq("user_id", &user_id)
        .execute()
        .await;

    if let Ok(resp) = resp_1 {
        let body = resp.text().await.unwrap();
        if body != "[]" {
            return "The spell you wanted to make already exists".to_string();
        }
    }

    let resp_2 = postgrest_client
        .from("monsters")
        .insert(json!(monster_data).to_string())
        .execute()
        .await;

    let _body = resp_2.unwrap().text().await.unwrap();

    colorize_println(format!("body: {:#?}", _body), Colors::BrightYellowFg);

    let data = Data {
        command: "command_create_monster".to_string(),
        monsters: monster_data
    };

    let data_json = json!(data).to_string();

    data_json
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("create_monster")
        .description("Creates a monster")
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
                .name("size")
                .description("The size of the monster")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("Tiny", "tiny")
                .add_string_choice("Small", "small")
                .add_string_choice("Medium", "medium")
                .add_string_choice("Large", "large")
                .add_string_choice("Huge", "huge")
                .add_string_choice("Gargantuan", "gargantuan")
        })
        .create_option(|option| {
            option
                .name("type")
                .description("The type of the monster")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("Aberration", "aberration")
                .add_string_choice("Beast", "beast")
                .add_string_choice("Celestial", "celestial")
                .add_string_choice("Construct", "construct")
                .add_string_choice("Dragon", "dragon")
                .add_string_choice("Elemental", "elemental")
                .add_string_choice("Fey", "fey")
                .add_string_choice("Fiend", "fiend")
                .add_string_choice("Giant", "giant")
                .add_string_choice("Humanoid", "humanoid")
                .add_string_choice("Monstrosity", "monstrosity")
                .add_string_choice("Ooze", "ooze")
                .add_string_choice("Plant", "plant")
                .add_string_choice("Undead", "undead")
        })
        .create_option(|option| {
            option
                .name("alignment")
                .description("The alignment of the monster")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("Lawful Good", "lawful_good")
                .add_string_choice("Neutral Good", "neutral_good")
                .add_string_choice("Chaotic Good", "chaotic_good")
                .add_string_choice("Lawful Neutral", "lawful_neutral")
                .add_string_choice("Neutral", "neutral")
                .add_string_choice("Chaotic Neutral", "chaotic_neutral")
                .add_string_choice("Lawful Evil", "lawful_evil")
                .add_string_choice("Neutral Evil", "neutral_evil")
                .add_string_choice("Chaotic Evil", "chaotic_evil")
        })
        .create_option(|option| {
            option
                .name("armor_class")
                .description("The armor class of the monster")
                .kind(CommandOptionType::String)
                .required(true)
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
                .name("challenge_rating")
                .description("The challenge rating of the monster")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

pub fn create_monster_embed(data: &str) -> CreateEmbed {
    let data: Data = serde_json::from_str(data).unwrap();

    let monster = data.monsters;

    CreateEmbed::default()
        .title(format!("Monster: {}", monster.name))
        .description("Created your monster! If you want to add abilities and such, you have to use a separate command for that.")
        .field("Size", monster.size, false)
        .field("Type", monster.type_, false)
        .field("Alignment", monster.alignment, false)
        .field("Armor Class", monster.armor_class, false)
        .field("Hit Points", monster.hit_points, false)
        .field("Challenge Rating", monster.challenge_rating, false)
        .to_owned()
}

pub fn create_second_embed() -> CreateEmbed {
    CreateEmbed::default()
        .title("The next step")
        .description("Now that you have created your monster, you can add more to it by running a few commands.")
        .field("Commands", "/add_speed\n/add_proficiencies\n/add_vulnerability\n/add_resistance\n/add_immunity\n/add_cimmunity\n/add_senses\n/add_language\n/add_special_abilities\n/add_actions\n/add_reactions\n/add_legendary_actions\n/add_lair_actions\n/add_description\n/add_spellcasting\n/add_spell", false)
        .to_owned()
}

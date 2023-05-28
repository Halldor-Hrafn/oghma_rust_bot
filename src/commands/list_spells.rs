use serenity::builder;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use postgrest::Postgrest;

use serde::{Serialize, Deserialize};

use colorized::*;

#[derive(Serialize, Deserialize, Debug)]
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
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    command: String,
    user_id: String,
    guild_id: String,
    spells: Vec<SpellData>,
}

pub async fn run(command: &ApplicationCommandInteraction) -> String {
    let user_id = command.user.id.to_string();
    let guild_id = command.guild_id.unwrap().to_string();

    let postgrest_client = Postgrest::new(std::env::var("SUPABASE_URL")
        .expect(colorize_this("Expected a URL in the environment", Colors::RedFg).as_str()).as_str())
        .insert_header("apikey", std::env::var("SUPABASE_PUBLIC_KEY")
        .expect(colorize_this("Expected an API key in the environment", Colors::RedFg).as_str()).as_str());

    let resp = postgrest_client
        .from("spells")
        .select("*")
        .eq("guild_id", guild_id.as_str())
        .eq("user_id", user_id.as_str())
        .execute()
        .await;

    let body = resp.unwrap().text().await.unwrap();

    println!("body: {:#?}", body);

    let deserialized_spells_vec: Vec<SpellData> = serde_json::from_str(&body).unwrap();

    let data = Data {
        command: "command_list_spells".to_string(),
        user_id: user_id,
        guild_id: guild_id,
        spells: deserialized_spells_vec,
    };

    let data_json: String = serde_json::to_string(&data).unwrap();

    data_json
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("list_spells")
        .description("List all spells you or someone else created in this server")
        .create_option(|option| {
            option
                .name("user")
                .description("User to list spells for")
                .kind(CommandOptionType::User)
                .required(false)
        })
}

pub fn create_list_spells_embed(content: &String) -> CreateEmbed {
    let data: Data = serde_json::from_str(content).unwrap();

    //copilot made the embed below
    // I have no fucking idea how it did it, but it works, so I won't complain.
    CreateEmbed::default()
        .title("Spell Created")
        .description(format!("A list of spells created by: {} in this guild", data.user_id))
        .fields(data.spells.iter().map(|spell| {
            (spell.name.clone(), format!("Level: {}\nCast Time: {}\nRange: {}\nComponents: {}\nDuration: {}\nSchool: {}\nAttack/Save: {}\nDamage/Effect: {}", spell.level, spell.cast_time, spell.range, spell.components, spell.duration, spell.school, spell.attack_save, spell.damage_effect), false)
        }).collect::<Vec<(String, String, bool)>>()).to_owned()
}

use serenity::builder;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use postgrest::Postgrest;

use serde::{Serialize, Deserialize};

use colorized::*;

#[derive(Serialize, Deserialize)]
struct MagicItemData {
    name: String,
    rarity: String,
    type_: String,
    description: String,
    attunement: bool,
    guild_id: String,
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    command: String,
    user_id: String,
    guild_id: String,
    magic_items: Vec<MagicItemData>,
}

pub async fn run(command: &ApplicationCommandInteraction) -> String {
    let user_id = command.user.id.to_string();
    let guild_id = command.guild_id.unwrap().to_string();

    let postgrest_client = Postgrest::new(std::env::var("SUPABASE_URL")
        .expect(colorize_this("Expected a URL in the environment", Colors::RedFg).as_str()).as_str())
        .insert_header("apikey", std::env::var("SUPABASE_PUBLIC_KEY")
        .expect(colorize_this("Expected an API key in the environment", Colors::RedFg).as_str()).as_str());

    let resp = postgrest_client
        .from("magic_items")
        .select("*")
        .eq("guild_id", guild_id.as_str())
        .eq("user_id", user_id.as_str())
        .execute()
        .await;

    let body = resp.unwrap().text().await.unwrap();

    colorize_println(format!("body: {:#?}", body), Colors::BrightYellowFg);

    let deserialized_magic_items_vec: Vec<MagicItemData> = serde_json::from_str(&body).unwrap();

    let data = Data {
        command: "command_list_magic_items".to_string(),
        user_id: user_id,
        guild_id: guild_id,
        magic_items: deserialized_magic_items_vec,
    };

    let data_json: String = serde_json::to_string(&data).unwrap();

    data_json
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("list_magic_items")
        .description("List all magic items you or someone else created in this server")
        .dm_permission(false)
        .create_option(|option| {
            option
                .name("user")
                .description("The user to list magic items for")
                .kind(CommandOptionType::User)
                .required(false)
        })
}
pub fn create_list_magic_items_embed(content: &String) -> CreateEmbed {
    let data: Data = serde_json::from_str(content).unwrap();

    CreateEmbed::default()
        .title("Magic Items created")
        .description(format!("Magic Items created by <@{}>", data.user_id))
        .fields(data.magic_items.iter().map(|magic_item| {
            (
                magic_item.name.clone(),
                format!("Rarity: {}\nType: {}\nDescription: {}\nAttunement: {}", magic_item.rarity, magic_item.type_, magic_item.description, magic_item.attunement),
                false
            )
        }).collect::<Vec<(String, String, bool)>>()).to_owned()
}
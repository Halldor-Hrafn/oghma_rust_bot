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
    magic_items: Vec<MagicItemData>
}

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
        .from("magic_items")
        .select("*")
        .eq("guild_id", guild_id.as_str())
        .eq("user_id", user)
        .eq("name", name)
        .execute()
        .await;

    let body = resp.unwrap().text().await.unwrap();

    colorize_println(format!("body: {:#?}", body), Colors::BrightYellowFg);

    let deserialized_magic_items_vec: Vec<MagicItemData> = serde_json::from_str(&body).unwrap();

    let data = Data {
        command: "command_list_magic_item".to_string(),
        magic_items: deserialized_magic_items_vec,
    };

    let data_json = serde_json::to_string(&data).unwrap();

    data_json
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("list_magic_item")
        .description("Lists one magic item")
        .dm_permission(false)
        .create_option(|option| {
            option
                .name("name")
                .description("The name for the magic item")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("user")
                .description("The user to list magic items for")
                .kind(CommandOptionType::User)
                .required(false)
        })
}

pub fn create_list_magic_item_embed(data: &str) -> builder::CreateEmbed {
    let data: Data = serde_json::from_str(data).unwrap();
    let magic = &data.magic_items[0];

    CreateEmbed::default()
        .title(magic.name.to_owned())
        .description(magic.description.to_owned())
        .field("Rarity", magic.rarity.to_owned(), false)
        .field("Type", magic.type_.to_owned(), false)
        .field("Attunement", magic.attunement.to_owned().to_string(), false)
        .to_owned()
}

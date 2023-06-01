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
struct MagicItemData {
    name: String,
    rarity: String,
    type_: String,
    description: String,
    attunement: String,
    guild_id: String,
    user_id: String,
}

#[derive(Serialize, Deserialize)]
struct Data {
    command: String,
    magic_items: MagicItemData
}

pub async fn run(command: &ApplicationCommandInteraction) -> String {
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
    let rarity = options
        .iter()
        .find(|option| option.name == "rarity")
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
    let description = options
        .iter()
        .find(|option| option.name == "description")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();

    let attunement: bool;
    match options.iter().find(|option| option.name == "attunement") {
        Some(option) => {
            attunement = option.value.as_ref().unwrap().as_bool().unwrap();
        },
        None => {
            attunement = false;
        }
    }

    let item_data = MagicItemData {
        name: name.to_string(),
        rarity: rarity.to_string(),
        type_: type_.to_string(),
        description: description.to_string(),
        attunement: attunement.to_string(),
        guild_id: guild_id.to_string(),
        user_id: user_id.to_string(),
    };

    let postgrest_client = Postgrest::new(std::env::var("POSTGREST_URL")
        .expect(colorize_this("Expected a URL in the environment", Colors::RedFg).as_str()).as_str())
        .insert_header("apikey", std::env::var("POSTGREST_PUBLIC_KEY")
        .expect(colorize_this("Expected an API key in the environment", Colors::RedFg).as_str()).as_str());

        let resp_1 = postgrest_client
        .from("magic_items")
        .select("*")
        .eq("name", name)
        .eq("guild_id", guild_id)
        .eq("user_id", user_id)
        .execute()
        .await;

    if let Ok(resp) = resp_1 {
        let body = resp.text().await.unwrap();
        if body != "[]" {
            return "The magic item you wanted to make already exists".to_string();
        }
    }

    let resp = postgrest_client
        .from("magic_items")
        .insert(json!(&item_data).to_string())
        .execute()
        .await;

    let _body = resp.unwrap().text().await.unwrap();

    colorize_println(format!("body: {:#?}", _body), Colors::BrightYellowFg);

    let data = Data {
        command: "command_create_magic_item".to_string(),
        magic_items: item_data
    };

    let data_json = serde_json::to_string(&data).unwrap();

    data_json
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> & mut builder::CreateApplicationCommand {
    command
        .name("create_magic_item")
        .description("Creates a new magic item")
        .dm_permission(false)
        .create_option(|option| {
            option
                .name("name")
                .description("The name of the magic item")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("rarity")
                .description("The rarity of the magic item")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("type")
                .description("The type of the magic item")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("description")
                .description("What the magic item does, the various versions of the magic item, etc.")
                .kind(CommandOptionType::String)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("attunement")
                .description("Whether the magic item requires attunement, default false")
                .kind(CommandOptionType::Boolean)
                .required(false)
        })
}

pub fn create_magic_item_embed(data: &str) -> builder::CreateEmbed {
    let data: Data = serde_json::from_str(data).unwrap();
    let magic_item = &data.magic_items;

    CreateEmbed::default()
        .title(format!("Magic Item *{}* has been created!", magic_item.name))
        .description(format!("{}", magic_item.description))
        .field("Name", magic_item.name.as_str(), false)
        .field("Rarity", magic_item.rarity.as_str(), false)
        .field("Type", magic_item.type_.as_str(), false)
        .field("Attunement", magic_item.attunement.as_str(), false)
        .color(0x00ff00)
        .to_owned()
}

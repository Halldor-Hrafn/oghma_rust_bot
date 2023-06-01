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
struct Data {
    command: String,
    command_name: String,
}

pub fn run(command: &ApplicationCommandInteraction) -> String {
    let options = &command.data.options;

    let command_name = options
        .iter()
        .find(|option| option.name == "command")
        .map_or("none", |option| option.value.as_ref().unwrap().as_str().unwrap());

    let data = Data {
        command: "command_help".to_string(),
        command_name: command_name.to_string(),
    };

    let data = serde_json::to_string(&data).unwrap();

    data
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("help")
        .description("Get help with a command")
        .create_option(|option| {
            option
                .name("command")
                .description("The command to get help with")
                .kind(CommandOptionType::String)
                .required(false)
        })
}

pub fn create_help_embed(data: &str) -> builder::CreateEmbed{
    let data: Data = serde_json::from_str(data).unwrap();

    let command = data.command_name.as_str();

    let embed = match command {
        "create_spell" => CreateEmbed::default()
            .title("Create Spell")
            .description("Creates a spell")
            .field("Usage", "```/create_spell [name] [level] [cast_time] [range] [components] [duration] [school] [attack/save] [damage/effect]```", false)
            .field("Example", "```/create_spell [Fireball] [3] [1 action] [150 feet] [V, S, M (a tiny ball of bat guano and sulfur] [Instantaneous] [Evocation] [Dexterity] [8d6 fire]```", false)
            .to_owned(),
        "list_spells" => CreateEmbed::default()
            .title("List Spells")
            .description("Lists all spells")
            .field("Usage", "```/list_spells (optional)[user]```", false)
            .field("Example", "```/list_spells```", false)
            .field("Example", "```/list_spells [user]```", false)
            .to_owned(),
        "list_spell" => CreateEmbed::default()
            .title("List Spell")
            .description("Lists a spell")
            .field("Usage", "```/list_spell [name] (optional)[user]```", false)
            .field("Example", "```/list_spell [Fireball]```", false)
            .field("Example", "```/list_spell [Fireball] [user]```", false)
            .to_owned(),
        "remove_spell" => CreateEmbed::default()
            .title("Remove Spell")
            .description("Removes a spell")
            .field("Usage", "```/remove_spell [name]```", false)
            .field("Example", "```/remove_spell [Fireball]```", false)
            .to_owned(),
        "create_magic_item" => CreateEmbed::default()
            .title("Create Magic Item")
            .description("Creates a magic item")
            .field("Usage", "```/create_magic_item [name] [rarity] [type]  [description] (optional, default false)[attunement]```", false)
            .field("Example", "```/create_magic_item [Ring of Fire Resistance] [Rare] [Ring] [You have resistance to fire damage while wearing this ring.]```", false)
            .to_owned(),
        "list_magic_items" => CreateEmbed::default()
            .title("List Magic Items")
            .description("Lists all magic items created by a user")
            .field("Usage", "```/list_magic_items (optional)[user]```", false)
            .field("Example", "```/list_magic_items```", false)
            .field("Example", "```/list_magic_items [user]```", false)
            .to_owned(),
        "list_magic_item" => CreateEmbed::default()
            .title("List Magic Item")
            .description("Lists a magic item")
            .field("Usage", "```/list_magic_item [name] (optional)[user]```", false)
            .field("Example", "```/list_magic_item [Ring of Fire Resistance]```", false)
            .field("Example", "```/list_magic_item [Ring of Fire Resistance] [user]```", false)
            .to_owned(),
        "remove_magic_item" => CreateEmbed::default()
            .title("Remove Magic Item")
            .description("Removes a magic item")
            .field("Usage", "```/remove_magic_item [name]```", false)
            .field("Example", "```/remove_magic_item [Ring of Fire Resistance]```", false)
            .to_owned(),
        _ => CreateEmbed::default()
            .title("Help")
            .description("Get help with a command")
            .field("Usage", "```/help [command]```", false)
            .field("Example", "```/help create_spell```", false)
            .field("Commands", "```create_spell\ncreate_spell\nlist_spells\nlist_spell\nremove_spell\ncreate_magic_item\nlist_magic_items\nlist_magic_item\nremove_magic_item```", false)
            .to_owned(),
    };

    embed
}

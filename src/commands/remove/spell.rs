use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use postgrest::Postgrest;

use dotenv::dotenv;

use colorized::*;

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

    let postgrest_client = Postgrest::new(std::env::var("POSTGREST_URL")
        .expect(colorize_this("Expected a URL in the environment", Colors::RedFg).as_str()).as_str())
        .insert_header("apikey", std::env::var("POSTGREST_PUBLIC_KEY")
        .expect(colorize_this("Expected an API key in the environment", Colors::RedFg).as_str()).as_str());

    let resp = postgrest_client
        .from("spells")
        .delete()
        .eq("guild_id", guild_id.as_str())
        .eq("user_id", user_id.as_str())
        .eq("name", name)
        .execute()
        .await;

    let _body = resp.unwrap().text().await.unwrap();

    colorize_println(format!("body: {:#?}", _body), Colors::BrightYellowFg);

    let response = format!("Spell *{}* removed", name);

    response
}

pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("remove_spell")
        .description("Removes a spell")
        .create_option(|option| {
            option
                .name("name")
                .description("The name of the spell to remove")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

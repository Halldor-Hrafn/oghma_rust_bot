mod commands;

use serenity::{async_trait};
use serenity::{prelude::*};

use serenity::model::prelude::*;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;

use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use dotenv::dotenv;
use std::env;

use colorized::*;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            colorize_println(format!("Got command: {:?}", command.data.name), Colors::YellowFg);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command),
                "welcome" => commands::welcome::run(&command),
                "roll" => commands::roll::run(&command),
                "create_spell" => commands::create_spell::run(&command).await,
                "create_magic_item" => commands::create_magic_item::run(&command).await,
                "list_spells" => commands::list_spells::run(&command).await,
                "list_magic_items" => commands::list_magic_items::run(&command).await,
                _ => "Unknown command".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            if content.as_str().contains("command_create_spells") {
                                let embed = commands::create_spell::create_spell_embed(&content);
                                message.add_embed(embed)
                            } else if content.as_str().contains("command_list_spells") {
                                let embed = commands::list_spells::create_list_spells_embed(&content);
                                message.add_embed(embed)
                            } else if content.as_str().contains("command_create_magic_item") {
                                let embed = commands::create_magic_item::create_magic_item_embed(&content);
                                message.add_embed(embed)
                            } else if content.as_str().contains("command_list_magic_items") {
                                let embed = commands::list_magic_items::create_list_magic_items_embed(&content);
                                message.add_embed(embed)
                            } else {
                                message.content(content)
                            }
                        })
                })
                .await
            {
                colorize_println(format!("Cannot respond to slash command: {:?}", why), Colors::RedFg)
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                colorize_println(format!("Error sending message: {:?}", why), Colors::RedFg)
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        colorize_println(format!("{} is connected!", ready.user.name), Colors::GreenFg);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect(colorize_this("Expected a guild id in the environment", Colors::RedFg).as_str())
                .parse::<u64>()
                .expect(colorize_this("Guild id is not a valid id", Colors::RedFg).as_str())
        );

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::welcome::register(command))
                .create_application_command(|command| commands::roll::register(command))
                .create_application_command(|command| commands::create_magic_item::register(command))
                .create_application_command(|command| commands::list_magic_items::register(command))
                //.create_application_command(|command| commands::create_spell::register(command))
                //.create_application_command(|command| commands::list_spells::register(command))
        }).await;

        colorize_println(format!("Registered guild commands: {:#?}", _commands), Colors::CyanFg);

        let _global_commands = Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::create_spell::register(command))
                .create_application_command(|command| commands::list_spells::register(command))
        }).await;

        colorize_println(format!("Registered global commands: {:#?}", _global_commands), Colors::CyanFg);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN")
        .expect(colorize_this("Expected a token in the environment", Colors::RedFg).as_str());
    let intents = GatewayIntents::all();

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .intents(intents)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        colorize_println(format!("Client error: {:?}", why), Colors::RedFg)
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

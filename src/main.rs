mod commands;

use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::{async_trait};
use serenity::{prelude::*};

use serenity::model::prelude::*;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;

use serenity::builder;

use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};

use std::error::Error;

use dotenv::dotenv;
use std::env;

use colorized::*;

#[group]
#[commands(ping)]
struct General;

struct Handler;

async fn handle_command(command: &ApplicationCommandInteraction) -> String {
    match command.data.name.as_str() {
        "help" => commands::help::run(&command),
        "ping" => commands::ping::run(&command),
        "roll" => commands::roll::run(&command),
        // spell commands
        "create_spell" => commands::create::spell::run(&command).await,
        "list_spells" => commands::list::spells::run(&command).await,
        "list_spell" => commands::list::spell::run(&command).await,
        "remove_spell" => commands::remove::spell::run(&command).await,
        // magic item commands
        "create_magic_item" => commands::create::magic_item::run(&command).await,
        "list_magic_items" => commands::list::magic_items::run(&command).await,
        "list_magic_item" => commands::list::magic_item::run(&command).await,
        "remove_magic_item" => commands::remove::magic_item::run(&command).await,
        // monster commands
        "create_monster" => commands::create::monster::run(&command).await,
        "list_monsters" => commands::list::monsters::run(&command).await,
        "list_monster" => commands::list::monster::run(&command).await,
        // add commands
        "add_speed" => commands::add::speed::run(&command).await,
        _ => "Unknown command".to_string(),
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            colorize_println(format!("Got command: {:?}", command.data.name), Colors::YellowFg);

            let content = handle_command(&command).await;

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(move |message| {
                            if content.as_str().contains("command_create_spells") {
                                let embed = commands::create::spell::create_embed(&content);
                                message.add_embed(embed)
                            } else if content.as_str().contains("command_list_spells") {
                                let embed = commands::list::spells::create_embed(&content);
                                message.add_embed(embed)
                            } else if content.as_str().contains("command_list_spell") {
                                let embed = commands::list::spell::create_embed(&content);
                                message.add_embed(embed)
                            } else if content.as_str().contains("command_create_magic_item") {
                                let embed = commands::create::magic_item::create_embed(&content);
                                message.add_embed(embed)
                            } else if content.as_str().contains("command_list_magic_items") {
                                let embed = commands::list::magic_items::create_embed(&content);
                                message.add_embed(embed)
                            } else if content.as_str().contains("command_list_magic_item") {
                                let embed = commands::list::magic_item::create_embed(&content);
                                message.add_embed(embed)
                            } else if content.as_str().contains("command_help") {
                                let embed = commands::help::create_embed(&content);
                                message.add_embed(embed)
                            } else if content.as_str().contains("command_create_monster") {
                                let embed = commands::create::monster::create_embed(&content);
                                let second_embed = commands::create::monster::create_second_embed();
                                message.add_embed(embed)
                                    .add_embed(second_embed)
                            } else if content.as_str().contains("command_list_monsters") {
                                let embed = commands::list::monsters::create_embed(&content);
                                message.add_embed(embed)
                            } else if content.as_str().contains("command_list_monster") {
                                let base_embed = commands::list::monster::create_embed(&content);
                                let speed_embed = commands::list::monster::create_speeds_embed(&content);
                                message
                                    .add_embed(base_embed)
                                    .add_embed(speed_embed)
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
        if msg.author.bot {
            return;
        }

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
                // .create_application_command(|command| commands::help::register(command))
                // .create_application_command(|command| commands::welcome::register(command))
                // .create_application_command(|command| commands::roll::register(command))
                // .create_application_command(|command| commands::create::monster::register(command))
                // .create_application_command(|command| commands::add::speed::register(command))
                // .create_application_command(|command| commands::list::monsters::register(command))
                // .create_application_command(|command| commands::list::spells::register(command))
                // .create_application_command(|command| commands::list::monster::register(command))
        }).await;

        colorize_println(format!("Registered guild commands: {:#?}", _commands), Colors::CyanFg);

        // let _global_commands = Command::set_global_application_commands(&ctx.http, |commands| {
        //     commands
        //         .create_application_command(|command| commands::help::register(command))
        //         .create_application_command(|command| commands::ping::register(command))
        //         // spell commands
        //         .create_application_command(|command| commands::create::spell::register(command))
        //         .create_application_command(|command| commands::list::spells::register(command))
        //         .create_application_command(|command| commands::list::spell::register(command))
        //         .create_application_command(|command| commands::remove::spell::register(command))
        //         // magic item commands
        //         .create_application_command(|command| commands::create::magic_item::register(command))
        //         .create_application_command(|command| commands::list::magic_items::register(command))
        //         .create_application_command(|command| commands::list::magic_item::register(command))
        //         .create_application_command(|command| commands::remove::magic_item::register(command))
        // }).await;

        // colorize_println(format!("Registered global commands: {:#?}", _global_commands), Colors::CyanFg);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_DEV_TOKEN")
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

#[command]
async fn test(ctx: &Context, msg: &Message) -> CommandResult {
    colorize_println(format!("{}", msg.content), Colors::GreenFg);
    msg.channel_id.say(&ctx.http, "Test!").await?;

    Ok(())
}

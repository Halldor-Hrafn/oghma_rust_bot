mod commands;

use serenity::{async_trait, client};
use serenity::{prelude::*};

use serenity::model::prelude::*;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;

use serenity::builder::CreateEmbed;

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

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Got command {:?}", command.data.name);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data),
                "numberinput" => commands::number_input::run(&command.data),
                "create" => commands::create::run(&command.data),
                "welcome" => commands::welcome::run(&command.data),
                "create_spells" => commands::create_spells::run(&command.data).await,
                _ => "Unknown command".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| {
                            if content.as_str().contains("command_create_spells") {
                                let embed = commands::create_spells::create_spell_embed(&content);
                                message.add_embed(embed)
                            } else {
                                message.content(content)
                            }
                        })
                })
                .await
            {
                println!("Cannot respond to slash command: {:?}", why);
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }

        if msg.content == "embed" {
            todo!()
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected a guild id in the environment")
                .parse::<u64>()
                .expect("Guild id is not a valid id")
        );

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::ping::register(command))
                .create_application_command(|command| commands::number_input::register(command))
                .create_application_command(|command| commands::welcome::register(command))
                .create_application_command(|command| commands::create::register(command))
                .create_application_command(|command| commands::create_spells::register(command))
        }).await;

        // println!("Registered commands: {:?}", commands);

        let _guild_commands = Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command)
        }).await;

        // println!("Registered guild commands: {:?}", guild_commands);
    }
}

// async fn send_embed(ctx: &Context, msg: &Message) {
//     let embed = CreateEmbed::default()
//         .title("Title")
//         .description("Description")
//         .field("Field 1", "Value 1", true)
//         .field("Field 2", "Value 2", true)
//         .footer(|f| f.text("Footer"));

//     let _ = msg.channel_id.send_message(&ctx.http, |m| {
//         m.embed(|e| {
//             e.clone_from(&embed);
//             e
//         })
//     }).await;
// }

#[tokio::main]
async fn main() {
    dotenv().ok();
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .group(&GENERAL_GROUP);

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");
    let intents = GatewayIntents::all();

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .intents(intents)
        .framework(framework)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

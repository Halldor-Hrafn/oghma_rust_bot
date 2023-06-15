use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use rand::Rng;

#[allow(dead_code)]
pub fn run(command: &ApplicationCommandInteraction) -> String {
    let data = &command.data;
    let options = &data.options;

    let dice = options
        .iter()
        .find(|option| option.name == "dice")
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();

    let result = roll_dice(dice);

    result.to_string()
}

#[allow(dead_code)]
pub fn register(command: &mut builder::CreateApplicationCommand) -> &mut builder::CreateApplicationCommand {
    command
        .name("roll")
        .description("Rolls dice")
        .create_option(|option| {
            option
                .name("dice")
                .description("The dice to roll")
                .kind(CommandOptionType::String)
                .required(true)
        })
}

fn roll_dice(dice: &str) -> i32 {
    let mut rng = rand::thread_rng();
    let mut sum = 0;

    for die in dice.split("+") {
        let parts: Vec<&str> = die.trim().split("d").collect();
        let num_dice = if parts[0].is_empty() {
            1
        } else {
            parts[0].parse::<i32>().unwrap()
        };
        let num_sides = parts[1].parse::<i32>().unwrap();

        for _ in 0..num_dice {
            sum += rng.gen_range(1..num_sides + 1);
        }
    }

    sum
}

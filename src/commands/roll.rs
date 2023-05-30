use serenity::builder;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use rand::Rng;

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

    let dice = dice.split("d").collect::<Vec<&str>>();

    println!("dice: {:#?}", dice);



    let num_dice = if dice[0].is_empty() {
        1
    } else {
        dice[0].parse::<i32>().unwrap()
    };
    let num_sides = dice[1].parse::<i32>().unwrap();

    let number = roll_dice(num_dice, num_sides);

    number.to_string()
}

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

fn roll_dice(num_dice: i32, num_sides: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let mut sum = 0;
    for _ in 0..num_dice {
        sum += rng.gen_range(1..num_sides + 1);
    }
    sum
}

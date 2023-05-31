# The Oghma rust bot

This is a bot that is created to assist Dungeon Masters with creating homebrew content for the 5th edition of D&D. It is currently in development and is not ready for use.

## Pre-requisites
- Rust
- Cargo
- PostgreSQL server (i.e Supabase)

## Installation

To install this project, you'll need to have Rust and Cargo installed, as well as a postgresql server. Once that is done, you can run:

```bash
git clone https://github.com/Halldor-Hrafn/oghma_rust_bot.git && cd oghma_rust_bot
```

Then, you need to create a `.env` file in the root of the project, and add the following variables:

```bash
DISCORD_TOKEN=<your discord bot token>
GUILD_ID=<your discord guild id>

POSTGREST_PUBLIC_KEY=<your postgrest public key>
POSTGREST_URL=<your postgrest url>
```

Then, you can run the following command to start the bot:

```bash
cargo run --release
```

## Features

This project has the following features:

- `ping`: A command that responds with "Pong!".
- `roll`: A command that rolls a specified number of dice with a specified number of sides.
- `create_spell`: A command that creates a new spell.
- `create_magic_item`: A command that creates a new magic item.
- `list_spells`: A command that lists all spells.
- `list_magic_items`: A command that lists all magic items.
- `remove_spell`: A command that removes a spell.
- `remove_magic_item`: A command that removes a magic item.

## License

This project is licensed under the GPL-3.0 License. See the `LICENSE` file for details.

## Contributing

If you want to contribute to the project, then you can fork the repository and create a pull request. If you want to add a new feature, then you should create a new branch with the name of the feature, and then create a pull request from that branch to the `master` branch.

## Author(s)

- [@Halldor-Hrafn](https://www.github.com/Halldor-Hrafn)

# Dönermann-Bot

A Repository with the Dönermann Bot used in a private Discord server of mine. It's using the Serenity rs libary. It can be forked and further used without any restrictions.

## Features

- Copy message of a citations channel to the database
- Show with a command how many were copied: `/citations`
- A normal ping command: `/döner`
- Manage birthdays with `/birthday set`, `/birthday delete`, and `/birthday next`
- Roll, claim, browse, delete, and rank waifus with `/waifu roll`, `/waifu collection`, and `/waifu leaderboard`
- Give an user a role when the user joins the server

## Commands

- `/citations`: Shows the latest recorded citation messages from the configured citations channel.
- `/döner`: Sends a Döner helper link.
- `/birthday set birth:<date>`: Saves your birthday using the configured `date_format`.
- `/birthday delete user:<user>`: Deletes your own birthday, or another user's birthday if you are listed as an admin.
- `/birthday next`: Shows the next upcoming birthdays.
- `/waifu roll`: Rolls a random MyAnimeList character from Jikan and adds a Claim button.
- `/waifu collection`: Shows your claimed waifus with Previous, Delete, and Next buttons.
- `/waifu leaderboard`: Ranks players by their total Goon Credits across claimed waifus.

## Usage

- First you have to add an Application in the **[Discord Developer Portal](https://discord.com/developers/applications)** and create a bot
- After that you paste your bot token in `config.toml` and have to enable all of the of the Privileged Gateway Intents options in the options of your bot
- Now you have to add the copy channel id and the id of the join role, ids of the admins, paths and date formatting to the `config.toml` file:

```toml
[bot]
# The Bot Token
token = ""
# A date format used for displaying dates inside the bot messages, default is `%d.%m.%Y`
date_format = "%d.%m.%Y"
# A timestamp format used for displaying timestamps inside the bot messages, default is `%d.%m.%Y %H:%M:%S`
timestamp_format = "%d.%m.%Y %H:%M:%S"

[paths]
# Path to the database file
database = "db.json"

[server]
# Id of the server
guild = ""
# Id of the citations channel
citations_channel = ""
# Id of the role which should be added on join
role_on_join = ""
# Id of the bot's admin, can be multiple
admins = [ "" ]
```

- You can start the bot by running the binary file provided in the release, make sure to give it the right permissions and that the `config.toml` is in the same directory as the binary:

```sh
./doenermann-bot
```

## Building Example (Cross)

```sh
cross build -r --target aarch64-unknown-linux-gnu
```

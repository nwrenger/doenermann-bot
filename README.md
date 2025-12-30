# Dönermann-Bot

A Repository with the Dönermann Bot used in a private Discord server of mine. It's using the Serenity rs libary. It can be forked and further used without any restrictions.

## Features

- Copy message of a channel to a file
- Show with a command how many were copied: `/count`
- A normal ping command: `/döner`
- Add your birthday: `/set_birthday`
- Show the next upcoming birthdays: `/next_birthdays`
- Delete a birthday of an user: `/delete_birthday`
- Give an user a role when the user joins the server

## Usage

- First you have to add an Application in the **[Discord Developer Portal](https://discord.com/developers/applications)** and create a bot
- After that you paste your bot token in `config.toml` and have to enable all of the of the Privileged Gateway Intents options in the options of your bot
- Now you have to add the copy channel id and the id of the join role, ids of the admins, paths and date formatting to the `config.toml` file:

```toml
[bot]
# The Bot Token
token = ""
# A date format used for dates inside the bot messages, default is `%d.%m.%Y`
date_format = "%d.%m.%Y"
# A timestamp format used for displaying timestamps inside the copied messages file, default is `%Y-%m-%d %H:%M:%S UTC`
timestamp_format = "%Y-%m-%d %H:%M:%S UTC"

[paths]
# Path to the birthdays file
birthdays = "birthdays.csv"
# Path to the messages file
messages = "citations.txt"

[server]
# Id of the copy channel
copy_channel = ""
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

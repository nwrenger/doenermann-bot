# Dönermann-Bot
A Reposetory with the Dönermann Bot used in a private Discord server of mine. It's using the Serenity rs libary. It can be forked and further used without any restrictions. 

Now it can:
- 
- Copy Message of a Channel in a file
- Show with a command how many were copied(/count)
- a normal ping(/döner) command
- Add your birthday with /set_birthday
- show the next upcomming birthdays(/next_birthdays)
- Give a user a Member Role when the user joins the server

Dependencies:
- 
- all Dependencies are Stated in the Cargo.toml
- just run the bin file provided in the release

Usage:
-
- First you have to add an Application in the Discord Developer Portal(https://discord.com/developers/applications) and create a bot 
- After that you paste your bot token in .env and have to enable all of the of the Privileged Gateway Intents options in the options of your bot
- Now you have to add your server id, the channel id(where you like the files to be copied from) and the id of the Member role(you have to create those) to the .env file:
```enviroment
DISCORD_TOKEN=Your Token
GUILD_ID=The Server Id
C_CHANNEL_ID=The copy Channel Id
ROLE_ID=The Role Id you give players when they join the server
```
- You can start the bot by running the binary file provided in the release(make sure to give it the right permissions and that the files: .env, birthdays.csv and citations.txt are in the same directory as the bin file):
```shell
./doenermann-bot
```

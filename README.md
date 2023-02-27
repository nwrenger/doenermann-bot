# Dönermann-Bot
A Reposetory with the Dönermann Bot used in a private Discord server of mine. It's using the discord.py libary. It can be forked and further used without any restrictions. 

Now it can:
- 
- Copy Message of a Channel in a file
- Show with a command how many were copied(/count)
- a normal ping(/döner) command
- a /stop command which will stop the bot, only the owner(discord-id) of the bot can dot that
- Add your birthday with /set_birthday
- show the next upcomming birthdays(/next_birthdays)
- Give a user a Birthday Role when the user has Birthday(Currently doesn't work because of API changes)
- Give a user a Member Role when the user joins the server

Dependencies:
-
- Python 3.11.1
- discord.py
- datetime(module)
- typing(module)
- pathlib(module)
- csv(module)

Usage:
-
- First you have to add an Application in the Discord Developer Portal(https://discord.com/developers/applications) and create a bot 
- After that you paste your bot token in token.env and have to enable all of the of the Privileged Gateway Intents options in the options of your bot
- Now you have to add your server id, your id, the channel id(where you like the files to be copied from), the id of the Birthday Role and the id of the Member role(you have to create those) to bot.py:
```python
server = discord.Object(id=YOUR_SERVER_ID)
owner = YOUR_ID
copied_channel = CHANNEL_ID
birthday_role = BIRTHDAY_ROLE_ID
member_role = MEMBER_ROLE_ID
```
- You can start the bot by running bot.py:
```shell
python3 bot.py
```

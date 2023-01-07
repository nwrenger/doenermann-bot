# Dönermann-Bot
A Reposetory with the Dönermann Bot used in a private Discord server of mine. It's using the discord.py libary. It can be forked and further used without any restrictions. 

Now it can:
- 
- Copy Message of a Channel in a file
- Show with a command how many were copied
- a normal ping(/döner) command
- a /stop command which will stop the bot, only the owner(discord-id) of the bot can dot that

Usage:
-
- First you have to add an Application in the Discord Developer Portal(https://discord.com/developers/applications) and create a bot 
- After that you paste your bot token in token.env and have to enable all of the of the Privileged Gateway Intents options in the options of your bot
- Now you have to add your server id, your id and the channel id(where you like the files to be copied from) to bot.py:
```python
server = discord.Object(id=YOUR_SERVER_ID)
owner = YOUR_ID
copied_channel = CHANNEL_ID
```
- Lastly you have to add the Path for the token.env file to bot.py:
```python
token = open("THE_PATH_OF_token.env_FILE", "r").read()
```
- And the Path for the citations.txt file to bot.py:
```python
file = open("THE_PATH_OF_citations.txt_FILE", "a")
```
- You can start the bot by running bot.py

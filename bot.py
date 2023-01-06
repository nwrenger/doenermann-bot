import datetime
import discord
from discord import app_commands

intents = discord.Intents.default()
intents.message_content = True
intents.members = True
token = open("THE_PATH_OF_token.env_FILE", "r").read()
counts = 0
server = discord.Object(id=YOUR_SERVER)
owner = YOUR_ID
copied_channel = CHANNEL_ID

class MyClient(discord.Client):
    def __init__(self, *, intents: discord.Intents):
        super().__init__(intents=intents)
        self.tree = app_commands.CommandTree(self)
    async def setup_hook(self):
        self.tree.copy_global_to(guild=server)
        await self.tree.sync(guild=server)

bot = MyClient(intents=intents)

file = open("THE_PATH_OF_citations.txt_FILE", "a")
file.write("Begin of Copying on " + str(datetime.datetime.now()) + ":\n")
file.flush()

@bot.event
async def on_ready():
    print(f'{bot.user} has connected to Discord!')


@bot.tree.command()
async def döner(interaction: discord.Interaction):
    """Ping?"""
    await interaction.response.send_message('Pong')

@bot.tree.command()
async def stop(interaction: discord.Interaction):
    """Shut the Bot down!"""
    if interaction.user.id == owner:
        await interaction.response.send_message('Shutting Down')
        exit()
    else:
        await interaction.response.send_message('You Dont have the permission to do that!')

@bot.tree.command()
async def count(interaction: discord.Interaction):
    """Gives the Count of the already Recorded Messages after last Start"""
    await interaction.response.send_message(counts)

@bot.event
async def on_message(message):
    # print(message)
    if message.channel.id == copied_channel:
        global counts
        counts = counts + 1
        line = str(message.content)
        # print(counts)
        user = str(message.author) + ": "
        print(user + line)
        file.write(user + line + "\n")
        file.flush()

bot.run(token)
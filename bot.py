from datetime import datetime, date
from discord.ext import tasks
from typing import Dict, List, Tuple
import discord
from discord import app_commands
from pathlib import Path

import csv

bot_dir = Path(__file__).parent
token_file = bot_dir / "token.env"
birthdays_file = bot_dir / "birthdays.csv"
citations_file = bot_dir / "citations.txt"

intents = discord.Intents.default()
intents.message_content = True
intents.members = True
token = token_file.read_text()
counts = 0
listofcounts = []
server = discord.Object(id=YOUR_SERVER_ID)
owner = YOUR_ID
copied_channel = CHANNEL_ID
birthday_role = BIRTHDAY_ROLE_ID
member_role = MEMBER_ROLE_ID
DATE_FMT = "%d.%m.%Y"

birthdays: Dict[int, date] = {}

if birthdays_file.exists():
    with birthdays_file.open() as f:
        reader = csv.DictReader(f)
        for row in reader:
            birth = datetime.strptime(row["birthday"], DATE_FMT).date()
            birthdays[int(row["user"])] = birth


class MyClient(discord.Client):
    def __init__(self, *, intents: discord.Intents):
        super().__init__(intents=intents)
        self.tree = app_commands.CommandTree(self)

    async def setup_hook(self):
        self.tree.copy_global_to(guild=server)
        await self.tree.sync(guild=server)


bot = MyClient(intents=intents)


citations = citations_file.open("a")
citations.write("%Begin of Copying on " + str(datetime.now()) + ":\n")
citations.flush()


@bot.event
async def on_ready():
    print(f'{bot.user} has connected to Discord!')


@bot.tree.command()
async def döner(interaction: discord.Interaction):
    """Döner bestellen?"""
    await interaction.response.send_message("Ne diggi, denkste ich habe das Geld dafür? Aber hier das sollte dir helfen: https://www.lieferando.de/lieferservice/doener/hannover-30159")


@bot.tree.command()
async def stop(interaction: discord.Interaction):
    """Shut the Bot down!"""
    if interaction.user.id == owner:
        e = discord.Embed(title="Shutting Down!",
                          description="", color=0x00ff15)
        await interaction.response.send_message(embed=e)
        exit()
    else:
        e = discord.Embed(
            title="You Dont have the permission to do that!", description="", color=0xff0000)
        await interaction.response.send_message(embed=e)


@bot.tree.command()
async def count(interaction: discord.Interaction):
    """Gives the Count of the already Recorded Messages after last Start"""
    e = discord.Embed(title="Already recorded messages: " +
                      str(counts) + "\nList of already recorded messages:", description="", color=0x00ff15)
    for line in listofcounts:
       e.add_field(name = "", value = line, inline = False)
    await interaction.response.send_message(embed=e)


@bot.tree.command()
async def next_birthdays(interaction: discord.Interaction):
    """The next 10 Upcomming Birthdays"""
    now = datetime.now().date()
    births_tmp: List[Tuple[int, date, date]] = []
    for user, birth in birthdays.items():
        next = calc_birthday(birth)
        births_tmp.append([user, birth, next])

    births_tmp.sort(key=lambda r: r[2])

    e = discord.Embed(title="Next Birthdays:", description="", color=0x0800ff)
    for user, birth, next in births_tmp[:10]:
        age = now.year - birth.year
        if [now.month, now.day] > [next.month, next.day]:
            age += 1
        e.add_field(name=next.strftime("%d %B %Y"),
                    value=f"<@{user}> ({age})", inline=False)
    await interaction.response.send_message(embed=e)


@bot.tree.command()
@app_commands.describe(
    birth="Your Birthday"
)
async def set_birthday(interaction: discord.Interaction, birth: str):
    """Set your Birhtday"""
    user = interaction.user.id
    try:
        birthdays[user] = datetime.strptime(birth, DATE_FMT).date()
    except Exception as ex:
        e = discord.Embed(title="Invalid date!",
                          description=str(ex), color=0xff0000)
        await interaction.response.send_message(embed=e)
        return

    with birthdays_file.open("w+") as csv_file:
        header = ["birthday", "user"]
        writer = csv.DictWriter(csv_file, fieldnames=header)
        writer.writeheader()
        for user, birth_date in birthdays.items():
            writer.writerow(
                {"birthday": birth_date.strftime(DATE_FMT), "user": user})

    e = discord.Embed(title="Your Birthday was set to: " +
                      birth, description="", color=0x0800ff)
    await interaction.response.send_message(embed=e)


@bot.event
async def on_message(message):
    # print(message)
    if message.channel.id == copied_channel:
        global counts
        global listofcounts
        counts = counts + 1
        line = str(message.content)
        # print(counts)
        user = str(message.author) + ": "
        print(user + line)
        listofcounts.append(user + line)
        citations.write(user + line + "\n")
        citations.flush()


#doesnt work, i will fix it later
# async def on_day():
#     now = datetime.now().date()
#     for user, birth in birthdays.items():
#         if calc_birthday(birth) == now:
#             await user.add_roles(user.guild.get_role(birthday_role))
#         else:
#             try:
#                 await birthday_role.delete()
#             except:
#                 return


@bot.event
async def on_member_join(user):
    await user.add_roles(user.guild.get_role(member_role))


def calc_birthday(birth: date) -> date:
    now = datetime.now().date()
    next = birth.replace(year=now.year)
    if next < now:
        next = next.replace(next.year + 1)
    return next



bot.run(token)

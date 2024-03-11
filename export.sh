#/bin/bash
cross build -r --target aarch64-unknown-linux-gnu
cp target/aarch64-unknown-linux-gnu/release/doenermann-bot ./
mkdir -p exp
zip -r exp/doenermann-bot.zip doenermann-bot .env citations.txt birthdays.csv
rm doenermann-bot

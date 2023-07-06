#/bin/bash
cargo build -r
cp target/release/doenermann-bot ./
zip -r exp/doenermann-bot_lin.zip doenermann-bot .env citations.txt birthdays.csv
rm doenermann-bot

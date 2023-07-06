#/bin/bash
cargo build -r
cp target/release/doenermann-bot ./
zip -r exp/doenermann-bot_v1.0.3.zip doenermann-bot .env citations.txt birthdays.csv
rm doenermann-bot

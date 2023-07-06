#/bin/bash
cargo build -r
cp target/release/doenermann-bot ./
zip -r exp/doenermann-bot_lin.zip doenermann-bot .env citations.txt birthdays.csv
rm doenermann-bot
cargo build -r --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/doenermann-bot.exe ./
zip -r exp/doenermann-bot_win.zip doenermann-bot.exe .env citations.txt birthdays.csv
rm doenermann-bot.exe
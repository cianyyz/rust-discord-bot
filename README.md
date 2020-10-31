

Rust Discord Bot
================

Written in Rust, using Serenity Discord Library.
Some Image processing is handled by Python using opencv.

```
virtualenv env
pip install -r requirements.txt
cargo build
cargo run
```

env variables
```
PREFIX=`
DISCORD_TOKEN=
RUST_LOG=debug
```

Bot invite w/ admin perms
https://discord.com/oauth2/authorize?client_id={YOUR_BOT_ID}&permissions=8&scope=bot
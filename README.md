# Discord-Webhook-CLI
A simple CLI for sending messages through discord web hooks. Useful for CI messages.

## Installation
Requires `cargo` (and hence the `rust` toolchain) installed on your system.
Then install from crates.io:
```bash
cargo install discord-webhook-cli
```

## Use
```bash
# NOTE: Will run dotenv and load from .env file if it exists, OVERWRITING CLI ARGS!
DISCORD_WEBHOOK_CLI_URL="https://your-url" DISCORD_WEBHOOK_CLI_MSG="cool message" DISCORD_WEBHOOK_CLI_USERNAME="Awesome Easy Bot" discord-webhook-cli
# Or you can use the flags
# PS: RUST_LOG exists for debugging purposes
RUST_LOG="trace" discord-webhook-cli --url "https://your-url" --msg "another cool message" --username "Awesome Easy Bot"
```


# AutoCoin Configuration Setup

## Problem Fixed

The AutoCoin trading bot was failing to start with error:
```
Failed to load settings: missing field `upbit`
```

This occurred because the program expected a configuration file at `.env/config.toml`.

## Solution

1. **Created `config.toml`** in the project root directory with all required sections:
   - `[upbit]` - Upbit API configuration
   - `[trading]` - Trading parameters
   - `[discord]` - Discord notification settings
   - `[system]` - System configuration
   - `[logging]` - Logging configuration

2. **Created `data`** directory for the SQLite database

## Before Running

You MUST update the placeholder values in `config.toml`:

### Upbit API Keys
```toml
[upbit]
access_key = "YOUR_ACTUAL_UPBIT_ACCESS_KEY"
secret_key = "YOUR_ACTUAL_UPBIT_SECRET_KEY"
```

Get your API keys from: https://upbit.com/my-api

### Discord Webhook (Optional)
If you want Discord notifications, set:
```toml
[discord]
enabled = true
webhook_url = "YOUR_DISCORD_WEBHOOK_URL"
```

## Running the Bot

1. Update `config.toml` with your actual API keys
2. Run using:
   ```bash
   ./run.sh
   ```
   or
   ```bash
   cargo run -- -c config.toml
   ```

## Environment Variables

The `.env` file is also supported for environment variable overrides:
- `UPBIT_ACCESS_KEY`
- `UPBIT_SECRET_KEY`
- `TRADING_TARGET_COINS`
- `TARGET_PROFIT_RATE`
- etc.
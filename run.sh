#!/bin/bash

# AutoCoin Trading Bot Startup Script

echo "Starting AutoCoin Trading Bot..."

# Check if config.toml exists
if [ ! -f "config.toml" ]; then
    echo "Error: config.toml not found!"
    echo "Please create config.toml with your API keys and settings."
    echo "A template config.toml has been created for you."
    exit 1
fi

# Check if .env exists for environment variables
if [ -f ".env" ]; then
    echo "Loading environment variables from .env..."
    export $(cat .env | xargs)
fi

# Create data directory if it doesn't exist
mkdir -p data

# Run the program
echo "Running with config: config.toml"
cargo run -- -c config.toml "$@"
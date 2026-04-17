#!/bin/bash
echo "Uninstalling mcp2cli..."

if [[ -f "/usr/local/bin/mcp2cli" ]]; then
    sudo rm /usr/local/bin/mcp2cli
    echo "Removed /usr/local/bin/mcp2cli"
fi

# Configuration directory
CONFIG_DIR="$HOME/Library/Application Support/com.mcp2cli.mcp2cli"
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    CONFIG_DIR="$HOME/.config/mcp2cli"
fi

if [[ -d "$CONFIG_DIR" ]]; then
    read -p "Do you want to remove configuration and cache at $CONFIG_DIR? [y/N] " confirm
    if [[ "$confirm" == "y" || "$confirm" == "Y" ]]; then
        rm -rf "$CONFIG_DIR"
        echo "Removed configuration directory."
    fi
fi

echo "Uninstallation complete."

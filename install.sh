#!/bin/bash
set -e

REPO="forechoandlook/mcp2cli"
LATEST_TAG=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')
VERSION_NUM=$(echo $LATEST_TAG | sed 's/^v//')

OS="linux"
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    OS="windows"
fi

BINARY_NAME="mcp2cli-$OS"
if [[ "$OS" == "windows" ]]; then
    BINARY_NAME="$BINARY_NAME.exe"
fi

echo "Installing mcp2cli $LATEST_TAG for $OS..."
URL="https://github.com/$REPO/releases/download/$LATEST_TAG/$BINARY_NAME"

curl -L -o mcp2cli_tmp "$URL"
chmod +x mcp2cli_tmp

if [[ "$OS" == "windows" ]]; then
    mv mcp2cli_tmp mcp2cli.exe
    echo "Installed to current directory as mcp2cli.exe"
else
    sudo mv mcp2cli_tmp ~/.local/bin/mcp2cli
    echo "Installed to ~/.local/bin/mcp2cli"
fi

echo "Installation complete. Try running: mcp2cli list"

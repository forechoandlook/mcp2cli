#!/bin/bash
set -e

REPO="forechoandlook/mcp2cli"
LATEST_TAG=$(curl -s https://api.github.com/repos/$REPO/releases/latest | grep '"tag_name":' | sed -E 's/.*"([^"]+)".*/\1/')

OS="linux"
if [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    OS="windows"
fi

ARCH=$(uname -m)
case $ARCH in
    x86_64) ARCH_NAME="x86_64" ;;
    arm64|aarch64) ARCH_NAME="arm64" ;;
    *) ARCH_NAME="x86_64" ;;
esac

BINARY_NAME="mcp2cli-$OS-$ARCH_NAME"
if [[ "$OS" == "windows" ]]; then
    BINARY_NAME="$BINARY_NAME.exe"
fi

echo "Installing mcp2cli $LATEST_TAG for $OS ($ARCH_NAME)..."
URL="https://github.com/$REPO/releases/download/$LATEST_TAG/$BINARY_NAME"

curl -L -o mcp2cli_tmp "$URL"
chmod +x mcp2cli_tmp

if [[ "$OS" == "windows" ]]; then
    mv mcp2cli_tmp mcp2cli.exe
    echo "Installed to current directory as mcp2cli.exe"
else
    mkdir -p ~/.local/bin
    mv mcp2cli_tmp ~/.local/bin/mcp2cli
    echo "Installed to ~/.local/bin/mcp2cli"
    echo "Make sure ~/.local/bin is in your PATH."
fi

echo "Installation complete. Try running: mcp2cli list"

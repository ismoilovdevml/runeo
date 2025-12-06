#!/bin/bash

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     RUNEO - O'rnatish Scripti          ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""

# Check if Rust is installed
if command -v rustc &> /dev/null; then
    RUST_VERSION=$(rustc --version)
    echo -e "${GREEN}✓ Rust allaqachon o'rnatilgan: ${RUST_VERSION}${NC}"
else
    echo -e "${YELLOW}Rust o'rnatilmoqda...${NC}"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
    echo -e "${GREEN}✓ Rust muvaffaqiyatli o'rnatildi${NC}"
fi

# Clone or update repository
REPO_URL="https://github.com/ismoilovdevml/runeo.git"
INSTALL_DIR="$HOME/.runeo"

if [ -d "$INSTALL_DIR" ]; then
    echo -e "${YELLOW}Runeo yangilanmoqda...${NC}"
    cd "$INSTALL_DIR"
    git pull origin main
else
    echo -e "${YELLOW}Runeo yuklab olinmoqda...${NC}"
    git clone "$REPO_URL" "$INSTALL_DIR"
    cd "$INSTALL_DIR"
fi

# Build release
echo -e "${YELLOW}Runeo kompilyatsiya qilinmoqda...${NC}"
cargo build --release

# Create symlink to bin
BINARY_PATH="$INSTALL_DIR/target/release/runeo"
BIN_DIR="$HOME/.local/bin"

mkdir -p "$BIN_DIR"

if [ -L "$BIN_DIR/runeo" ]; then
    rm "$BIN_DIR/runeo"
fi

ln -s "$BINARY_PATH" "$BIN_DIR/runeo"

# Setup .env file
if [ ! -f "$INSTALL_DIR/.env" ]; then
    cp "$INSTALL_DIR/.env.example" "$INSTALL_DIR/.env"
    echo -e "${YELLOW}⚠ .env fayli yaratildi. Iltimos, API kalitingizni kiriting:${NC}"
    echo -e "${BLUE}  $INSTALL_DIR/.env${NC}"
fi

# Check if bin is in PATH
if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
    echo ""
    echo -e "${YELLOW}PATH ga qo'shish uchun quyidagini ~/.bashrc yoki ~/.zshrc ga qo'shing:${NC}"
    echo -e "${BLUE}  export PATH=\"\$HOME/.local/bin:\$PATH\"${NC}"
    echo ""
fi

echo ""
echo -e "${GREEN}╔════════════════════════════════════════╗${NC}"
echo -e "${GREEN}║   ✓ Runeo muvaffaqiyatli o'rnatildi!   ║${NC}"
echo -e "${GREEN}╚════════════════════════════════════════╝${NC}"
echo ""
echo -e "${BLUE}Ishga tushirish:${NC}"
echo -e "  1. .env fayliga API kalitingizni kiriting:"
echo -e "     ${YELLOW}nano $INSTALL_DIR/.env${NC}"
echo -e "  2. Runeo ni ishga tushiring:"
echo -e "     ${YELLOW}runeo${NC}"
echo ""

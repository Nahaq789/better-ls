#!/bin/bash

INSTALL_DIR="$HOME/.local/bin"
CONFIG_DIR="$HOME/.lscd"
SCRIPT_PATH="$CONFIG_DIR/lscd.sh"

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

mkdir -p "$INSTALL_DIR"
mkdir -p "$CONFIG_DIR"

echo -e "${YELLOW}Building Rust program...${NC}"

cargo build --release

cp target/release/better-ls "$INSTALL_DIR/"

cat > "$SCRIPT_PATH" << 'EOF'
#!/bin/bash

lscd_function() {
    local start_path="${1:-.}"
    
    local temp_file=$(mktemp)
    ~/.local/bin/better-ls "$start_path" | tee "$temp_file"
    
    local target_dir=$(tail -n 1 "$temp_file")
    
    rm -f "$temp_file"
    
    if [ -n "$target_dir" ] && [ -d "$target_dir" ]; then
        cd "$target_dir"
        echo -e "\nMoved to: $target_dir"
    else
        echo -e "\nNo valid directory was selected"
        return 1
    fi
}

# Execute the function
lscd_function "$@"
EOF

chmod +x "$SCRIPT_PATH"

SHELL_CONFIG=""
if [ -n "$ZSH_VERSION" ] || [ -f "$HOME/.zshrc" ]; then
    SHELL_CONFIG="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ] || [ -f "$HOME/.bashrc" ]; then
    SHELL_CONFIG="$HOME/.bashrc"
fi

if [ -n "$SHELL_CONFIG" ]; then
    echo -e "${YELLOW}Adding shell configuration...${NC}"
    
    if grep -q "# lscd function" "$SHELL_CONFIG"; then
        echo "lscd configuration already exists. Updating..."
        sed -i '/# lscd function/,/# end of lscd function/d' "$SHELL_CONFIG"
    fi
    
    cat >> "$SHELL_CONFIG" << EOF

lscd() {
    source "$SCRIPT_PATH" "\$@"
}
EOF

    echo -e "${GREEN}Installation complete!${NC}"
    echo -e "Restart your terminal or run the following command:"
    echo -e "  ${YELLOW}source $SHELL_CONFIG${NC}"
    echo -e "\nUsage: Run ${GREEN}lscd [path]${NC} to display folder list in the specified path (defaults to current directory)."
else
    echo "No supported shell configuration file found."
    echo "Please manually add the following to your shell configuration file:"
    echo ""
    echo "lscd() {"
    echo "    source \"$SCRIPT_PATH\" \"\$@\""
    echo "}"
fi

# lscd - Interactive Directory Navigation Tool

## Overview
`lscd` is a command-line tool that enhances directory navigation by providing an interactive way to browse and change directories. It combines the functionality of `ls` and `cd` commands, allowing you to see directory contents and navigate to them with simple keystrokes.

## Features
- Display directories in the current or specified path
- Navigate to directories with simple key selections
- Skip files and only show directories for navigation
- Easy to install and use

## Installation

### Prerequisites
- Rust and Cargo (cargo 1.81.0)
- Bash or Zsh shell

### Automatic Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/Nahaq789/better-ls.git
   cd better-ls
   ```

2. Run the installation script:
   ```bash
   chmod +x install.sh
   ./install.sh
   ```

3. Restart your terminal or source your shell configuration:
   ```bash
   source ~/.bashrc   # For Bash
   # OR
   source ~/.zshrc    # For Zsh
   ```

### Manual Installation
If the automatic installation doesn't work for you, follow these steps:

1. Build the Rust program:
   ```bash
   cargo build --release
   ```

2. Copy the binary to a location in your PATH:
   ```bash
   mkdir -p ~/.local/bin
   cp target/release/better-ls ~/.local/bin/
   ```

3. Create the shell function in your `.bashrc` or `.zshrc`:
   ```bash
   echo '
   # lscd function
   lscd() {
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
   # end of lscd function
   ' >> ~/.bashrc   # Or ~/.zshrc for Zsh
   ```

4. Reload your shell configuration:
   ```bash
   source ~/.bashrc   # Or ~/.zshrc for Zsh
   ```

## Usage

### Basic Usage
Simply type `lscd` to see directories in the current location:
```bash
lscd
```

### Specify a Starting Path
You can also specify a starting path:
```bash
lscd ~/Documents
```

### Navigation
1. The tool will display available directories with key mappings (a, b, c, etc.)
2. Type the key corresponding to the directory you want to navigate to
3. Press Enter to confirm your selection
4. You will be moved to the selected directory

## Troubleshooting

### Command Not Found
If you get a "command not found" error, make sure:
- You've sourced your shell configuration after installation
- The installation script completed successfully
- The `~/.local/bin` directory is in your PATH

### Permission Issues
If you encounter permission issues:
```bash
chmod +x ~/.lscd/lscd.sh
chmod +x ~/.local/bin/better-ls
```

## Uninstallation
To uninstall the tool:
1. Remove the binary:
   ```bash
   rm ~/.local/bin/better-ls
   ```

2. Remove the shell script:
   ```bash
   rm -rf ~/.lscd
   ```

3. Remove the function from your shell configuration by editing your `.bashrc` or `.zshrc` file and removing the lines between `# lscd function` and `# end of lscd function`.
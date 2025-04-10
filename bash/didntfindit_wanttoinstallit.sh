#!/bin/bash

# Function to show script usage
usage() {
    echo "Usage: $0 <package_name>"
    echo "Search and install packages from official repositories and AUR"
    exit 1
}

# Check if an argument is provided
if [ $# -eq 0 ]; then
    usage
fi

# Capture the package to install
COMMAND="$1"

# Function to install package
install_package() {
    local package="$1"
    local manager="$2"
    
    echo "Package '$package' found in $manager. Do you want to install it? [Y/n]"
    read -r RESPONSE
    
    if [[ "$RESPONSE" =~ ^[Yy]$ || -z "$RESPONSE" ]]; then
        if [ "$manager" == "pacman" ]; then
            sudo pacman -S --noconfirm "$package"
        elif [ "$manager" == "yay" ]; then
            yay -S --noconfirm "$package"
        fi
        
        if [ $? -eq 0 ]; then
            echo "Installation of $package completed successfully."
        else
            echo "There was an error installing $package."
        fi
    else
        echo "Installation cancelled."
    fi
}

# Search in official repositories
if pacman -Si "$COMMAND" >/dev/null 2>&1; then
    echo "Package '$COMMAND' is available in the official repositories."
    install_package "$COMMAND" "pacman"
    exit 0
fi

# Search in AUR using yay
if command -v yay >/dev/null 2>&1; then
    if yay -Si "$COMMAND" >/dev/null 2>&1; then
        echo "Package '$COMMAND' is available in the AUR."
        install_package "$COMMAND" "yay"
        exit 0
    fi
fi

# Double-check for installed package
if pacman -Q "$COMMAND" >/dev/null 2>&1; then
    echo "Package '$COMMAND' is already installed."
    exit 0
fi

# If package is not found
echo "No package named '$COMMAND' found in official repositories or AUR."
exit 1

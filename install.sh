#!/bin/bash

# Script URL: https://raw.githubusercontent.com/samuelgja/gen/main/install.sh
# Install script "curl -sSL https://raw.githubusercontent.com/samuelgja/gen/main/install.sh | sh"

# Gen Installation Script

echo "Installing Gen..."

# Specify the URL to the latest release of Gen
GEN_RELEASE_URL="https://github.com/samuelgja/gen/raw/main/releases/latest/download/gen"


# Create a 'gen' directory in the user's home directory
GEN_INSTALL_DIR="$HOME/gen"

echo "Downloading Gen from $GEN_RELEASE_URL"
echo "Destination: $GEN_INSTALL_DIR"
# Download the Gen binary
curl -L $GEN_RELEASE_URL -o $GEN_INSTALL_DIR

# # Make the binary executable
chmod +x $GEN_INSTALL_DIR

# Optionally, add the installation directory to the user's PATH.
# Note: This will be effective only in the current shell session.
# To make it permanent, add the following line to the .bashrc, .zshrc, or equivalent
# export PATH=$PATH:$GEN_INSTALL_DIR
# but before export check if exsit
if [[ ":$PATH:" != *":$GEN_INSTALL_DIR:"* ]]; then
    echo "export PATH=$PATH:$GEN_INSTALL_DIR" >> ~/.bashrc
    echo "export PATH=$PATH:$GEN_INSTALL_DIR" >> ~/.zshrc
fi


echo "Gen was installed successfully to $GEN_INSTALL_DIR"
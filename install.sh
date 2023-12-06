#!/bin/bash

# Script URL: https://raw.githubusercontent.com/samuelgja/gen/main/install.sh

# Gen Installation Script

echo "Installing Gen..."

# Specify the URL to the latest release of Gen
GEN_BINARY="https://github.com/samuelgja/gen/raw/main/releases/latest/download/gen"

# Download the Gen binary
curl -L $GEN_BINARY -o gen

# # Make the binary executable
# chmod +x gen

# # Move the binary to a directory in the user's PATH
# # /usr/local/bin is a common directory for user-installed binaries
# mv gen /usr/local/bin/

# echo "Gen has been installed successfully."
# echo "Type 'gen --help' to see available commands."
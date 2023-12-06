#!/bin/bash

# Gen Installation Script

echo "Downloading Gen from https://github.com/samuelgja/gen/releases..."

# Specify the URL to the latest release of Gen
GEN_RELEASE_URL="https://github.com/samuelgja/gen/releases/latest/download/gen"

# Download the Gen binary
curl -L $GEN_RELEASE_URL -o gen

# Make the binary executable
chmod +x gen

# Move the binary to a directory in the user's PATH
# /usr/local/bin is a common directory for user-installed binaries
mv gen /usr/local/bin/

echo "Gen has been installed successfully."
echo "Type 'gen --help' to see available commands."
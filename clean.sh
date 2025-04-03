#!/bin/bash

# Define the directory to scan; default to current directory if not provided
SCAN_DIR="${1:-.}"

# Find and remove all 'Cargo.lock' files
find "$SCAN_DIR" -type f -name 'Cargo.lock' -exec rm -f {} +

# Find and remove all 'target' directories
find "$SCAN_DIR" -type d -name 'target' -exec rm -rf {} +

echo "Cleanup completed."

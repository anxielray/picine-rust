#!/bin/bash

# Check if both arguments are provided
if [ $# -ne 2 ]; then
  echo "Usage: $0 <file_name> \"<commit_message>\""
  exit 1
fi

# Assign arguments to variables
FILE_NAME=$1
COMMIT_MESSAGE=$2

# Stage the specified file
git add "$FILE_NAME"

# Create the commit message
COMMIT_MSG="[Data]: {Add commit to the piscine rust repository. $COMMIT_MESSAGE}"

# Commit with the formatted message
git commit -m "$COMMIT_MSG"

git push

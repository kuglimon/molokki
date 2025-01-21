#!/usr/bin/env bash

# Check if a command was provided
if [ -z "$1" ]; then
  exit 1
fi

# File to store the output
OUTPUT_FILE="$HOME/.steam-debugger.log"

echo "$@" > "$OUTPUT_FILE"

# MANGOHUD_CONFIG=fps_limit=60 mangohud
# Execute the command and capture its output
"$@" >> "$OUTPUT_FILE" 2>&1
EXIT_CODE=$?

# Return the exit code of the executed command
exit $EXIT_CODE

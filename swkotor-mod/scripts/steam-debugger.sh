#!/usr/bin/env bash
# TODO(tatu): Nixify this crap
#
# Simple utility to make debugging apps under proton a bit easier for me.

# Check if a command was provided
if [ -z "$1" ]; then
  exit 1
fi

# File to store the output
OUTPUT_FILE="$HOME/.steam-debugger.log"

echo "Starting debugging with command:" > "$OUTPUT_FILE"
echo "$@" >> "$OUTPUT_FILE"

# Show dll load attempts. Use this sparingly. Opening kotor for a couple of
# seconds generated 500MB of logs.
# export WINEDEBUG=+relay

# Force steam to load our modified dll
export WINEDLLOVERRIDES="dinput8.dll=n"

# MANGOHUD_CONFIG=fps_limit=60 mangohud
# Execute the command and capture its output
"$@" >> "$OUTPUT_FILE" 2>&1
EXIT_CODE=$?

# Return the exit code of the executed command
exit $EXIT_CODE

#!/bin/zsh
shopt -s expand_aliases
source ~/.zshrc

FEATURE_FILE="current_feature.md"

while true; do
    if [[ ! -f "$FEATURE_FILE" ]]; then
        echo "Error: $FEATURE_FILE not found" >&2
        sleep 60
        continue
    fi

    message=$(<"$FEATURE_FILE")
    message="${message//$'\n'/ }"   # collapse newlines into spaces
    message="${message## }"          # trim leading spaces
    message="${message%% }"          # trim trailing spaces

    if [[ -z "$message" ]]; then
        echo "Error: $FEATURE_FILE is empty" >&2
        sleep 60
        continue
    fi

    push "$message"
    sleep 60
done
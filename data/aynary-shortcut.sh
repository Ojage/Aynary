#!/bin/bash
# Script to handle keyboard shortcut for Aynary dictionary
# This script gets the primary selection and sends it to Aynary via DBus

# Get primary selection (highlighted text)
WORD=$(xclip -o -selection primary 2>/dev/null || echo "")

if [ -z "$WORD" ]; then
    # Try clipboard if primary selection is empty
    WORD=$(xclip -o 2>/dev/null || echo "")
fi

if [ -n "$WORD" ]; then
    # Extract first word
    WORD=$(echo "$WORD" | awk '{print $1}' | sed 's/[^a-zA-Z0-9]//g')
    
    if [ -n "$WORD" ]; then
        # Call Aynary via DBus
        dbus-send --session --type=method_call \
            --dest=com.aynary.Dictionary \
            /com/aynary/Dictionary \
            com.aynary.Dictionary.LookupAndShow \
            string:"$WORD" 2>/dev/null
    fi
fi


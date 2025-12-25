# Aynary Browser Extension

This browser extension adds "Define with Aynary" to the context menu when text is selected.

## Installation

### Chrome/Chromium

1. Open Chrome and navigate to `chrome://extensions/`
2. Enable "Developer mode"
3. Click "Load unpacked"
4. Select the `browser-extension` directory

### Firefox

1. Open Firefox and navigate to `about:debugging`
2. Click "This Firefox"
3. Click "Load Temporary Add-on"
4. Select `manifest.json` from the `browser-extension` directory

## Native Host Setup

The extension requires the native messaging host to be installed:

1. Install Aynary (which installs `aynary-native-host`)
2. Install the native host manifest:
   - Chrome: Copy `native_host.json` to `~/.config/google-chrome/NativeMessagingHosts/com.aynary.dictionary.json`
   - Firefox: Copy `native_host.json` to `~/.mozilla/native-messaging-hosts/com.aynary.dictionary.json`
3. Update the path in the JSON file to point to `/usr/local/bin/aynary-native-host` (or your installation path)
4. Update the `allowed_origins` with your extension ID (found in `chrome://extensions/` or `about:debugging`)

## Usage

1. Highlight text on any webpage
2. Right-click to open the context menu
3. Select "Define with Aynary"
4. The Aynary dictionary window will open with the definition


# Installation Guide for Aynary Dictionary

## Step 1: Install System Dependencies

On Fedora 43, run:
```bash
sudo dnf install rust gtk4-devel libadwaita-devel pkgconf-pkg-config dbus-devel
```

This installs:
- `rust` - Rust compiler and cargo
- `gtk4-devel` - GTK4 development libraries
- `libadwaita-devel` - libadwaita development libraries  
- `pkgconf-pkg-config` - pkg-config tool for finding libraries
- `dbus-devel` - DBus development libraries

## Step 2: Build the Application

From the project directory:
```bash
cd aynary
cargo build --release
```

This will compile the application and create binaries in `target/release/`:
- `aynary` - Main dictionary application
- `aynary-native-host` - Native messaging host for browser extension

Note: The dictionary is fully offline. The bundled data in `data/dictionary.json` is embedded at build time; edit that file before building if you want to ship a larger dataset.

## Step 3: Install the Application

To install system-wide:
```bash
sudo make install
```

To install to your local directory (~/.local):
```bash
make PREFIX=$HOME/.local install
```

This will:
- Install binaries to `/usr/local/bin` (or `~/.local/bin`)
- Install desktop files
- Install DBus service files
- Set up browser extension native messaging hosts
- Install systemd user service file

## Step 4: (Optional) Enable Clipboard Monitor Service

If you want the clipboard monitor to run automatically:
```bash
systemctl --user enable aynary-clipboard-monitor.service
systemctl --user start aynary-clipboard-monitor.service
```

Note: Clipboard monitoring is currently disabled in the code. The keyboard shortcut script is the recommended way to lookup words.

## Step 5: (Optional) Set Up Keyboard Shortcut

1. Open GNOME Settings
2. Go to Keyboard → Keyboard Shortcuts
3. Add a custom shortcut:
   - Name: "Lookup with Aynary"
   - Command: `/usr/local/bin/aynary-shortcut` (or `~/.local/bin/aynary-shortcut` if installed locally)
   - Shortcut: Ctrl+Shift+D (or your preferred key combination)

## Step 6: (Optional) Install Browser Extension

### Chrome/Chromium
1. Open `chrome://extensions/`
2. Enable "Developer mode"
3. Click "Load unpacked"
4. Select the `browser-extension` directory

### Firefox
1. Open `about:debugging`
2. Click "This Firefox"
3. Click "Load Temporary Add-on"
4. Select `manifest.json` from the `browser-extension` directory

After loading the extension, update the extension ID in the native messaging host JSON files (see browser-extension/README.md for details).

## Troubleshooting

### Build Errors

If you get errors about missing libraries:
- Make sure all dependencies from Step 1 are installed
- Verify with: `pkg-config --exists gtk4 && echo "GTK4 found"`

### Runtime Errors

If the application doesn't start:
- Check that GTK4 and libadwaita are installed: `dnf list installed | grep -E "(gtk4|libadwaita)"`
- Try running from the terminal to see error messages: `./target/release/aynary`

### DBus Errors

If DBus communication fails:
- Make sure the DBus service file is installed: `ls ~/.local/share/dbus-1/services/com.aynary.Dictionary.service` (or `/usr/share/dbus-1/services/`)
- Check if the service is running: `dbus-send --session --print-reply --dest=com.aynary.Dictionary /com/aynary/Dictionary org.freedesktop.DBus.Introspectable.Introspect`


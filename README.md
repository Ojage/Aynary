# Aynary Dictionary

A native dictionary application for Fedora Workstation 43 with comprehensive context menu integration.

## Features

- **Standalone Dictionary App**: Open Aynary to search for word definitions
- **Browser Integration**: Right-click highlighted text in browsers to define with Aynary
- **Global Clipboard Monitoring**: Automatically lookup words when you highlight text
- **Keyboard Shortcut**: Use Ctrl+Shift+D to lookup selected text

## Building

### Prerequisites

- Rust toolchain (rustc, cargo)
- GTK4 development libraries
- libadwaita
- pkg-config

On Fedora:
```bash
sudo dnf install rust gtk4-devel libadwaita-devel pkg-config
```

### Build

```bash
cargo build --release
```

## Installation

Install system-wide:
```bash
sudo make install
```

Or install to a custom prefix:
```bash
make PREFIX=$HOME/.local install
```

## Usage

### Standalone Application

Simply run `aynary` from the command line or launch it from the application menu.

### Browser Extension

1. Install the browser extension from `browser-extension/` directory
2. The extension will add a "Define with Aynary" option to the right-click context menu
3. Highlight text and right-click to define

### Keyboard Shortcut

1. Install the application
2. Set up the keyboard shortcut in GNOME Settings (or use the provided script)
3. Highlight text anywhere and press Ctrl+Shift+D

### Clipboard Monitoring

The clipboard monitor runs automatically when the application starts. It watches for text selections and automatically looks up single words.

## Development

### Project Structure

```
aynary/
├── src/
│   ├── main.rs              # Application entry point
│   ├── app.rs               # Main application logic
│   ├── ui.rs                # UI components
│   ├── api.rs               # Dictionary API client
│   ├── dbus_service.rs      # DBus IPC service
│   ├── clipboard_monitor.rs # Clipboard monitoring
│   ├── shortcut_handler.rs  # Keyboard shortcut handling
│   └── native_host.rs       # Browser extension native host
├── data/                    # Desktop integration files
├── browser-extension/       # Browser extension files
└── Makefile                 # Build and install system
```

## License

MIT OR Apache-2.0


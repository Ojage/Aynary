PREFIX ?= /usr/local
BINDIR ?= $(PREFIX)/bin
DATADIR ?= $(PREFIX)/share
APPDIR ?= $(DATADIR)/applications
DBUSDIR ?= $(DATADIR)/dbus-1/services
SYSTEMDDIR ?= $(HOME)/.config/systemd/user
SCHEMADIR ?= $(DATADIR)/glib-2.0/schemas
NATIVEHOSTDIR ?= $(HOME)/.config/google-chrome/NativeMessagingHosts
NATIVEHOSTDIR_FIREFOX ?= $(HOME)/.mozilla/native-messaging-hosts

.PHONY: all build install clean

all: build

build:
	cargo build --release

install: build
	# Install binary
	install -Dm755 target/release/aynary $(DESTDIR)$(BINDIR)/aynary
	install -Dm755 target/release/aynary-native-host $(DESTDIR)$(BINDIR)/aynary-native-host
	
	# Install desktop entry
	install -Dm644 data/com.aynary.Dictionary.desktop $(DESTDIR)$(APPDIR)/com.aynary.Dictionary.desktop
	
	# Install DBus service file
	install -Dm644 data/com.aynary.Dictionary.service $(DESTDIR)$(DBUSDIR)/com.aynary.Dictionary.service
	
	# Install systemd service (user service)
	mkdir -p $(DESTDIR)$(SYSTEMDDIR)
	install -Dm644 data/aynary-clipboard-monitor.service $(DESTDIR)$(SYSTEMDDIR)/aynary-clipboard-monitor.service
	
	# Install GSettings schema
	install -Dm644 data/org.gnome.settings-daemon.plugins.media-keys.aynary.gschema.xml $(DESTDIR)$(SCHEMADIR)/org.gnome.settings-daemon.plugins.media-keys.aynary.gschema.xml
	glib-compile-schemas $(DESTDIR)$(SCHEMADIR)
	
	# Install keyboard shortcut script
	install -Dm755 data/aynary-shortcut.sh $(DESTDIR)$(BINDIR)/aynary-shortcut
	
	# Install browser extension native host
	mkdir -p $(DESTDIR)$(NATIVEHOSTDIR)
	sed "s|/usr/local/bin|$(BINDIR)|g" browser-extension/native_host.json > $(DESTDIR)$(NATIVEHOSTDIR)/com.aynary.dictionary.json
	
	mkdir -p $(DESTDIR)$(NATIVEHOSTDIR_FIREFOX)
	sed "s|/usr/local/bin|$(BINDIR)|g" browser-extension/native_host.json > $(DESTDIR)$(NATIVEHOSTDIR_FIREFOX)/com.aynary.dictionary.json
	
	# Update desktop database
	update-desktop-database $(DESTDIR)$(APPDIR) 2>/dev/null || true

clean:
	cargo clean

uninstall:
	rm -f $(DESTDIR)$(BINDIR)/aynary
	rm -f $(DESTDIR)$(BINDIR)/aynary-native-host
	rm -f $(DESTDIR)$(BINDIR)/aynary-shortcut
	rm -f $(DESTDIR)$(APPDIR)/com.aynary.Dictionary.desktop
	rm -f $(DESTDIR)$(DBUSDIR)/com.aynary.Dictionary.service
	rm -f $(DESTDIR)$(SYSTEMDDIR)/aynary-clipboard-monitor.service
	rm -f $(DESTDIR)$(SCHEMADIR)/org.gnome.settings-daemon.plugins.media-keys.aynary.gschema.xml
	rm -f $(DESTDIR)$(NATIVEHOSTDIR)/com.aynary.dictionary.json
	rm -f $(DESTDIR)$(NATIVEHOSTDIR_FIREFOX)/com.aynary.dictionary.json
	update-desktop-database $(DESTDIR)$(APPDIR) 2>/dev/null || true
	glib-compile-schemas $(DESTDIR)$(SCHEMADIR) 2>/dev/null || true


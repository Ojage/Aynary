use crate::dbus_service;
use std::process::Command;

pub struct ShortcutHandler;

impl ShortcutHandler {
    pub fn new() -> Self {
        Self
    }

    /// Register a global keyboard shortcut via GNOME gsettings
    /// This creates a custom keybinding that calls the aynary-shortcut script
    pub fn register_shortcut(&self, shortcut: &str) -> Result<(), Box<dyn std::error::Error>> {
        // On GNOME, global keyboard shortcuts are managed via gsettings
        // This method would programmatically set up the shortcut
        // For now, users set it up manually via GNOME Settings or use:
        //
        // gsettings set org.gnome.settings-daemon.plugins.media-keys custom-keybindings \
        //   "['/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/aynary/']"
        //
        // gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/aynary/ \
        //   name 'Lookup with Aynary'
        // gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/aynary/ \
        //   command 'aynary-shortcut'
        // gsettings set org.gnome.settings-daemon.plugins.media-keys.custom-keybinding:/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/aynary/ \
        //   binding '<Primary><Shift>d'
        
        // For now, return Ok - manual setup is more reliable
        Ok(())
    }

    /// Handle a keyboard shortcut trigger
    /// This gets the primary clipboard selection and triggers a lookup
    pub fn handle_shortcut_trigger(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get primary selection using xclip (standard Linux tool)
        // This approach works reliably across X11 and many Wayland compositors
        let output = Command::new("xclip")
            .args(&["-o", "-selection", "primary"])
            .output();

        let word = match output {
            Ok(output) if output.status.success() => {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_matches(|c: char| !c.is_alphanumeric())
                    .to_string()
            }
            _ => {
                // Fallback to regular clipboard
                Command::new("xclip")
                    .args(&["-o"])
                    .output()
                    .ok()
                    .and_then(|o| {
                        if o.status.success() {
                            Some(String::from_utf8_lossy(&o.stdout)
                                .trim()
                                .split_whitespace()
                                .next()
                                .unwrap_or("")
                                .trim_matches(|c: char| !c.is_alphanumeric())
                                .to_string())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default()
            }
        };

        if !word.is_empty() && word.len() < 50 {
            // Trigger lookup via DBus
            dbus_service::lookup_and_show_via_dbus(&word)?;
        }

        Ok(())
    }

    /// Setup application-level shortcuts (when the app window is focused)
    /// This can be used for shortcuts that work within the application
    pub fn setup_app_shortcuts(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Application-level shortcuts can be set up using gtk::ShortcutController
        // This is useful for shortcuts that only work when the app has focus
        // For now, we focus on global shortcuts via the shell script approach
        // Future enhancement: integrate with GTK4 ShortcutController for in-app shortcuts
        
        Ok(())
    }
}

impl Default for ShortcutHandler {
    fn default() -> Self {
        Self::new()
    }
}


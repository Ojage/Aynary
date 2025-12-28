use adw::prelude::*;
use adw::ApplicationWindow;
use gtk4::prelude::*;
use gtk4::{Entry, ScrolledWindow, TextView, WrapMode, Orientation, Box as GtkBox, AboutDialog};

pub struct AppWindow {
    pub window: adw::ApplicationWindow,
    pub search_entry: Entry,
    pub definition_view: TextView,
}

impl AppWindow {
    pub fn new(app: &adw::Application) -> Self {
        // Create main window
        let window = adw::ApplicationWindow::builder()
            .application(app)
            .title("Aynary")
            .default_width(600)
            .default_height(700)
            .build();

        // Create header bar with search
        let header = adw::HeaderBar::new();
        let search_entry = Entry::builder()
            .placeholder_text("Search for a word...")
            .hexpand(true)
            .build();

        header.set_title_widget(Some(&search_entry));
        
        // Enable window controls (close, minimize, maximize buttons) - these appear automatically
        header.set_show_end_title_buttons(true);
        header.set_show_start_title_buttons(true);
        
        // Add About button with menu icon
        let about_button = gtk4::Button::from_icon_name("open-menu-symbolic");
        about_button.set_tooltip_text(Some("About Aynary"));
        let app_clone = app.clone();
        about_button.connect_clicked(move |_| {
            // Show about dialog directly
            show_about_dialog_ui(&app_clone);
        });
        header.pack_end(&about_button);

        // Create definition display area
        let definition_view = TextView::builder()
            .editable(false)
            .cursor_visible(false)
            .wrap_mode(WrapMode::Word)
            .top_margin(12)
            .bottom_margin(12)
            .left_margin(12)
            .right_margin(12)
            .build();

        // Text view is already set to non-editable via builder

        let scrolled = ScrolledWindow::builder()
            .child(&definition_view)
            .vexpand(true)
            .hexpand(true)
            .build();

        // Create main content box
        let content = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .build();

        content.append(&header);
        content.append(&scrolled);

        window.set_content(Some(&content));

        Self {
            window,
            search_entry,
            definition_view,
        }
    }

    pub fn set_definition(&self, text: &str) {
        let buffer = self.definition_view.buffer();
        // Note: GTK TextBuffer doesn't support Pango markup directly
        // We'll use plain text for now, markup can be added later if needed
        buffer.set_text(text);
    }

    pub fn get_search_text(&self) -> String {
        self.search_entry.text().to_string()
    }

    pub fn show(&self) {
        self.window.present();
    }

    pub fn hide(&self) {
        self.window.set_visible(false);
    }

    pub fn set_loading(&self, loading: bool) {
        if loading {
            self.definition_view.buffer().set_text("Loading...");
        }
    }
}

fn show_about_dialog_ui(app: &adw::Application) {
    if let Some(win) = app.active_window() {
        let about = AboutDialog::builder()
            .program_name("Aynary")
            .version("0.1.0")
            .copyright("© 2026 Salathiel Ojage")
            .license_type(gtk4::License::MitX11)
            .authors(vec!["Salathiel Ojage".to_string()])
            .comments("A modern dictionary application for Fedora Workstation\n\nDeveloped by Salathiel Ojage")
            .build();
        
        about.set_transient_for(Some(&win));
        about.present();
    }
}


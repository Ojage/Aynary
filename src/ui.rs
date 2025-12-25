use adw::prelude::*;
use adw::ApplicationWindow;
use gtk::prelude::*;
use gtk::{Entry, ScrolledWindow, TextView};

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
        header.set_show_end_title_buttons(false);
        header.set_show_start_title_buttons(false);

        // Create definition display area
        let definition_view = TextView::builder()
            .editable(false)
            .cursor_visible(false)
            .wrap_mode(gtk::WrapMode::Word)
            .top_margin(12)
            .bottom_margin(12)
            .left_margin(12)
            .right_margin(12)
            .build();

        // Enable markup in the text view
        definition_view.buffer().set_editable(false);

        let scrolled = ScrolledWindow::builder()
            .child(&definition_view)
            .vexpand(true)
            .hexpand(true)
            .build();

        // Create main content box
        let content = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
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


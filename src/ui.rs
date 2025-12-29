use adw::prelude::*;
use adw::{ApplicationWindow, Clamp, WindowTitle};
use gtk4::prelude::*;
use gtk4::{
    AboutDialog,
    Align,
    Box as GtkBox,
    Entry,
    EntryIconPosition,
    Image,
    Label,
    Orientation,
    ScrolledWindow,
    Stack,
    TextView,
    WrapMode,
};

pub struct AppWindow {
    pub window: adw::ApplicationWindow,
    pub search_entry: Entry,
    pub definition_view: TextView,
    content_stack: Stack,
}

impl AppWindow {
    pub fn new(app: &adw::Application) -> Self {
        // Create main window
        let window = adw::ApplicationWindow
            ::builder()
            .application(app)
            .title("Aynary")
            .default_width(600)
            .default_height(700)
            .build();

        // Create header bar with search
        let header = adw::HeaderBar::new();
        header.add_css_class("flat");
        let title = WindowTitle::new("Aynary", "Dictionary");
        header.set_title_widget(Some(&title));

        let search_entry = Entry::builder()
            .placeholder_text("Search words, phrases, and more...")
            .hexpand(true)
            .build();
        search_entry.set_icon_from_icon_name(
            EntryIconPosition::Primary,
            Some("system-search-symbolic"),
        );
        search_entry.add_css_class("search-entry");

        // Enable window controls (close, minimize, maximize buttons) - these appear automatically
        header.set_show_end_title_buttons(true);
        header.set_show_start_title_buttons(true);

        // Add About button with menu icon
        let about_button = gtk4::Button::from_icon_name("help-about-symbolic");
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
        definition_view.add_css_class("definition-text");

        // Text view is already set to non-editable via builder

        let scrolled = ScrolledWindow::builder()
            .child(&definition_view)
            .vexpand(true)
            .hexpand(true)
            .build();

        let placeholder_icon = Image::from_icon_name("system-search-symbolic");
        placeholder_icon.add_css_class("placeholder-icon");
        placeholder_icon.set_pixel_size(48);

        let placeholder_title = Label::new(Some("Ready when you are"));
        placeholder_title.add_css_class("placeholder-title");
        placeholder_title.set_halign(Align::Center);

        let placeholder_subtitle =
            Label::new(Some("Search above to see definitions, examples, and usage."));
        placeholder_subtitle.add_css_class("placeholder-subtitle");
        placeholder_subtitle.set_halign(Align::Center);
        placeholder_subtitle.set_wrap(true);
        placeholder_subtitle.set_justify(gtk4::Justification::Center);

        let placeholder_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(12)
            .halign(Align::Center)
            .valign(Align::Center)
            .build();
        placeholder_box.append(&placeholder_icon);
        placeholder_box.append(&placeholder_title);
        placeholder_box.append(&placeholder_subtitle);

        let content_stack = Stack::builder()
            .hexpand(true)
            .vexpand(true)
            .build();
        content_stack.add_named(&placeholder_box, Some("placeholder"));
        content_stack.add_named(&scrolled, Some("definition"));
        content_stack.set_visible_child_name("placeholder");

        let definition_card = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .vexpand(true)
            .hexpand(true)
            .build();
        definition_card.add_css_class("definition-card");
        definition_card.append(&content_stack);

        let brand_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(6)
            .build();
        let brand_title = Label::new(Some("Aynary"));
        brand_title.add_css_class("brand-title");
        brand_title.set_halign(Align::Start);
        let brand_subtitle = Label::new(Some("A modern dictionary for fast, focused lookups."));
        brand_subtitle.add_css_class("brand-subtitle");
        brand_subtitle.set_halign(Align::Start);
        brand_subtitle.set_wrap(true);
        brand_box.append(&brand_title);
        brand_box.append(&brand_subtitle);

        // Create main content box
        let content = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .spacing(16)
            .margin_top(24)
            .margin_bottom(24)
            .margin_start(24)
            .margin_end(24)
            .build();
        content.append(&brand_box);
        content.append(&search_entry);
        content.append(&definition_card);

        let clamp = Clamp::builder()
            .maximum_size(640)
            .tightening_threshold(520)
            .build();
        clamp.set_child(Some(&content));

        // Create main content container
        let main_box = GtkBox::builder()
            .orientation(Orientation::Vertical)
            .build();
        main_box.append(&header);
        main_box.append(&clamp);

        window.set_content(Some(&main_box));

        Self {
            window,
            search_entry,
            definition_view,
            content_stack,
        }
    }

    pub fn set_definition(&self, text: &str) {
        let buffer = self.definition_view.buffer();
        // Note: GTK TextBuffer doesn't support Pango markup directly
        // We'll use plain text for now, markup can be added later if needed
        buffer.set_text(text);
        if text.trim().is_empty() {
            self.content_stack.set_visible_child_name("placeholder");
        } else {
            self.content_stack.set_visible_child_name("definition");
        }
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
            self.content_stack.set_visible_child_name("definition");
            self.definition_view.buffer().set_text("Loading...");
        }
    }
}

fn show_about_dialog_ui(app: &adw::Application) {
    if let Some(win) = app.active_window() {
        let credits_text =
            r#"Credits

            Aynary is created and maintained by Salathiel Ojage.
            This application is built using open-source technologies and libraries from the Linux and free-software community. Special thanks to the developers and maintainers whose work makes projects like this possible.
            Dictionary data and linguistic references are sourced from publicly available and open lexical resources.
            Aynary is released under the MIT License, allowing free use, modification, and distribution."#;

        let comments_text =
            format!("A modern dictionary application for Fedora Workstation\n\n{}", credits_text);

        let about = AboutDialog::builder()
            .program_name("Aynary")
            .version("0.1.0")
            .copyright("© 2026 Salathiel Ojage")
            .license_type(gtk4::License::MitX11)
            .authors(vec!["Salathiel Ojage".to_string()])
            .comments(&comments_text)
            .build();

        about.set_transient_for(Some(&win));
        about.present();
    }
}
